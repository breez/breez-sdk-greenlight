use crate::{
    error::{SdkError, SdkResult},
    LogEntry, LogStream,
};
use anyhow::Result;
use chrono::Local;
use env_logger::Logger;
use log::{Log, Metadata, Record};
use once_cell::sync::OnceCell;
use std::io::Write;
use std::{fs::OpenOptions, sync::RwLock};

pub(crate) static SDK_LOGGER: OnceCell<RwLock<GlobalSdkLogger>> = OnceCell::new();

/// Wrapper that redirects all incoming log statements to the [GlobalSdkLogger]
struct RustLogger {}

impl log::Log for RustLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Some(l) = SDK_LOGGER.get() {
                l.read().unwrap().log(record);
            }
        }
    }

    fn flush(&self) {}
}

pub(crate) struct GlobalSdkLogger {
    /// SDK internal logger, which logs to file
    pub(crate) file_logger: Option<Logger>,
    /// Optional external logger, that can receive a stream of log statements
    pub(crate) log_listener: Option<Box<dyn LogStream>>,
}

impl GlobalSdkLogger {
    fn log(&self, record: &Record) {
        if let Some(logger) = &self.log_listener {
            logger.log(LogEntry {
                line: record.args().to_string(),
                level: record.level().as_str().to_string(),
            });
        }
        if let Some(logger) = &self.file_logger {
            logger.log(record);
        }
    }
}

pub(crate) fn init_logger() -> Result<RwLock<GlobalSdkLogger>> {
    let logger = GlobalSdkLogger {
        file_logger: None,
        log_listener: None,
    };
    log::set_boxed_logger(Box::new(RustLogger {}))?;
    log::set_max_level(log::LevelFilter::Debug);
    Ok(RwLock::new(logger))
}

pub(crate) fn build_env_logger(log_dir: &str) -> SdkResult<Logger> {
    let target_log_file = Box::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{log_dir}/sdk.log"))
            .map_err(|_| SdkError::InitFailed {
                err: "Can't create log file".into(),
            })?,
    );
    let logger = env_logger::Builder::new()
        .target(env_logger::Target::Pipe(target_log_file))
        .parse_filters(
            r#"
                info,
                gl_client=warn,
                h2=warn,
                hyper=warn,
                breez_sdk_core::reverseswap=info,
                lightning_signer=warn,
                reqwest=warn,
                rustls=warn,
                rustyline=warn,
                vls_protocol_signer=warn
            "#,
        )
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.module_path().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .build();

    Ok(logger)
}
