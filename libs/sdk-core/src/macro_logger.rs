/// Log at the given level.
#[macro_export]
macro_rules! log {
    ($logger:expr, $lvl:expr, $($arg:tt)*) => {
        $logger.log($crate::LogMessage::new($lvl, &format!($($arg)*), module_path!(), file!(), line!()));
    };
}

/// Log at the `ERROR` level.
#[macro_export]
macro_rules! log_error {
    ($logger:expr, $($arg:tt)*) => {
        $crate::log!($logger, $crate::LogLevel::Error, $($arg)*);
    };
}

/// Log at the `WARN` level.
#[macro_export]
macro_rules! log_warn {
    ($logger:expr, $($arg:tt)*) => {
        $crate::log!($logger, $crate::LogLevel::Warn, $($arg)*);
    };
}

/// Log at the `INFO` level.
#[macro_export]
macro_rules! log_info {
    ($logger:expr, $($arg:tt)*) => {
        $crate::log!($logger, $crate::LogLevel::Info, $($arg)*);
    };
}

/// Log at the `DEBUG` level.
#[macro_export]
macro_rules! log_debug {
    ($logger:expr, $($arg:tt)*) => {
        $crate::log!($logger, $crate::LogLevel::Debug, $($arg)*);
    };
}

/// Log at the `TRACE` level.
#[macro_export]
macro_rules! log_trace {
    ($logger:expr, $($arg:tt)*) => {
        $crate::log!($logger, $crate::LogLevel::Trace, $($arg)*);
    };
}

/// A generic logging macro that logs at the specified level. If a logger is provided,
/// it will use it (global log by default).
#[macro_export]
macro_rules! log_or_debug_generic {
    ($logger:expr, $lvl:expr, $message:expr, $($args:tt)*) => {
        if let Some(logger) = $logger {
            $crate::log!(logger, $lvl, $message, $($args)*);
        } else {
            match $lvl {
                $crate::LogLevel::Error => error!($message, $($args)*),
                $crate::LogLevel::Warn => warn!($message, $($args)*),
                $crate::LogLevel::Info => info!($message, $($args)*),
                $crate::LogLevel::Debug => debug!($message, $($args)*),
                $crate::LogLevel::Trace => trace!($message, $($args)*),
            }
        }
    };
}
