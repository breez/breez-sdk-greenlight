// Copied the classic rust log crate without the global behavior.
// Unlike the initial crate, the log line has no "enabled" function.
// It is left to the user to add filter logic to the 'log' implementation.

use core::fmt;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

static LOG_LEVEL_NAMES: [&str; 5] = ["ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

/// An enum representing the available verbosity levels of the logger.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub enum LogLevel {
    /// Designates very serious errors
    Error,
    /// Designates hazardous situations
    Warn,
    /// Designates useful information
    Info,
    /// Designates lower priority information
    Debug,
    /// Designates very low priority, often extremely verbose, information
    Trace,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", LOG_LEVEL_NAMES[*self as usize])
    }
}

/// A struct representing a log message.
#[derive(Clone, Debug)]
pub struct LogMessage {
    /// The verbosity level of the message.
    pub level: LogLevel,
    /// The log message. Since Uniffi does not support
    /// Rust-specific types like fmt::Arguments, we should
    /// use simpler types that can be serialized and deserialized.
    pub message: String,
    /// The module path of the message.
    pub module_path: String,
    /// The source file containing the message.
    pub file: String,
    /// The line containing the message.
    pub line: u32,
}

impl LogMessage {
    pub fn new(
        level: LogLevel,
        message: &str,
        module_path: &str,
        file: &str,
        line: u32,
    ) -> LogMessage {
        LogMessage {
            level,
            message: message.to_string(),
            module_path: module_path.to_string(),
            file: file.to_string(),
            line,
        }
    }
}

/// A trait encapsulating the operations required of a logger
/// Unlike the initial crate, the log line has no "enabled" function.
/// It is left to the user to add filter logic to the 'log' implementation.
pub trait Logger: Send + Sync {
    /// Logs the message with the specified log level, module path, file, and line.
    fn log(&self, message: LogMessage);
}

// A No-Op logger
pub struct NopLogger;

impl Logger for NopLogger {
    fn log(&self, _: LogMessage) {}
}

// ClassicLogger is a logger that will implicitly use the classic rust log crate.
// It is useful when having static function but we don't want to pass a logger to it.
pub struct ClassicLogger;

impl Logger for ClassicLogger {
    fn log(&self, message: LogMessage) {
        match message.level {
            LogLevel::Error => error!("{} - {}", message.module_path, message.message),
            LogLevel::Warn => warn!("{} - {}", message.module_path, message.message),
            LogLevel::Info => info!("{} - {}", message.module_path, message.message),
            LogLevel::Debug => debug!("{} - {}", message.module_path, message.message),
            LogLevel::Trace => trace!("{} - {}", message.module_path, message.message),
        }
    }
}

/// FileSystemLogger represents a file-based logger.
pub struct FileSystemLogger {
    // We use Arc to share ownership of the LineWriter in a lightly way
    // among multiple structures or threads without cloning the
    // heavy file handle. We ensure thread safety using Mutex.
    file: Arc<Mutex<io::LineWriter<fs::File>>>,
    level: LogLevel,
}

impl FileSystemLogger {
    pub fn new(file_path: &str, level: LogLevel) -> Result<FileSystemLogger, io::Error> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)?;

        let file = io::LineWriter::new(file);
        let file = Arc::new(Mutex::new(file));

        Ok(FileSystemLogger { file, level })
    }
}

impl Logger for FileSystemLogger {
    fn log(&self, message: LogMessage) {
        if message.level <= self.level {
            if let Ok(mut file) = self.file.lock() {
                // Right format?
                writeln!(
                    &mut *file,
                    "[{}] {}: {} - {}",
                    message.level,
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    message.module_path,
                    message.message
                )
                .expect("Failed to write to log file");
            }
        }
    }
}

/// MultiLogger implements Logger trait and can contain various loggers.
pub struct MultiLogger {
    loggers: Vec<Box<dyn Logger>>,
}

impl MultiLogger {
    pub fn new(loggers: Vec<Box<dyn Logger>>) -> MultiLogger {
        MultiLogger { loggers }
    }
}

impl Logger for MultiLogger {
    fn log(&self, message: LogMessage) {
        for logger in &self.loggers {
            logger.log(message.clone());
        }
    }
}

// StdoutLogger is logger that will log directly to the STDOUT stream.
pub struct StdoutLogger {
    level: LogLevel,
}

impl StdoutLogger {
    pub fn new(level: LogLevel) -> StdoutLogger {
        StdoutLogger { level }
    }
}

impl Logger for StdoutLogger {
    fn log(&self, message: LogMessage) {
        if message.level <= self.level {
            // I chose to use the locker instead stdout, but is it worth it?
            let stdout = io::stdout();
            let mut stdout = stdout.lock();

            // Right format?
            writeln!(
                &mut stdout,
                "[{}] {}: {} - {}",
                message.level,
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                message.module_path,
                message.message
            )
            .expect("Failed to write to stdout");
        }
    }
}

#[cfg(test)]
mod tests {
    // Import necessary modules and crates.
    use super::*;
    use crate::test_utils::TestLogger;
    use crate::{log_debug, log_error, log_info, log_trace, log_warn};

    #[test]
    fn test_macros_integration() {
        let logger = TestLogger::new(LogLevel::Info);

        // Use the macros to generate log records with various log levels.
        log_error!(logger, "Error message");
        log_warn!(logger, "Warning message");
        log_info!(logger, "Info message");
        log_debug!(logger, "Debug message");
        log_trace!(logger, "Trace message");

        // Get the log data from the logger.
        let log_data = logger.get_log_data();
        let borrowed_data = log_data.lock().unwrap();

        // Verify that the log data contains the right messages for the right level.
        assert_eq!(
            borrowed_data.get(&format!("{:?}", LogLevel::Error)),
            Some(&"Error message".to_string())
        );
        assert_eq!(
            borrowed_data.get(&format!("{:?}", LogLevel::Warn)),
            Some(&"Warning message".to_string())
        );
        assert_eq!(
            borrowed_data.get(&format!("{:?}", LogLevel::Info)),
            Some(&"Info message".to_string())
        );
        assert_eq!(borrowed_data.get(&format!("{:?}", LogLevel::Debug)), None); // Not logged
        assert_eq!(borrowed_data.get(&format!("{:?}", LogLevel::Trace)), None); // Not logged
    }

    #[test]
    fn test_log_level_order() {
        let expected_order = [
            LogLevel::Error,
            LogLevel::Warn,
            LogLevel::Info,
            LogLevel::Debug,
            LogLevel::Trace,
        ];

        for (i, &level) in expected_order.iter().enumerate() {
            assert_eq!(i as u32, level as u32, "Enum variant order is incorrect");
        }
    }
}
