use crate::models::LevelFilter as BindingLevelFilter;
use anyhow::{anyhow, Result};
use env_logger::{Builder, Logger, Target};

use log::{
    max_level, set_boxed_logger, set_max_level, LevelFilter, Log, Metadata, Record,
    STATIC_MAX_LEVEL,
};

use std::fs::OpenOptions;
use std::io::Write;

use chrono::Utc;

/* env_logger */

const ENV_LOGGER_FILTER: &str = r#"
debug,
breez_sdk_core::backup=info,
breez_sdk_core::breez_services=info,
breez_sdk_core::input_parser=warn,
breez_sdk_core::persist::reverseswap=info,
breez_sdk_core::reverseswap=info,
sdk_common=debug,
gl_client::node=info,
gl_client::node::service=info,
gl_client::tls=info,
gl_client::scheduler=info,
gl_client::signer=info,
gl_client=debug,
h2=warn,
h2::client=info,
h2::codec::framed_read=warn,
h2::codec::framed_write=warn,
h2::proto::connection=info,
h2::proto::settings=info,
hyper=warn,
hyper::client::connect::dns=info,
hyper::client::connect::https=info,
lightning_signer=warn,
lightning_signer::node=warn,
reqwest=warn,
reqwest::connect=warn,
rustls=warn,
rustls::anchors=warn,
rustls::conn=warn,
rustls::client::common=warn,
rustls::client::hs=warn,
rustls::client::tls13=warn,
rustyline=warn,
rusqlite_migration=warn,
tower::buffer::worker=warn,
vls_protocol_signer=warn,
vls_protocol_signer::handler::HandlerBuilder::do_build=warn
"#;

pub fn init_env_logger(target: Option<Target>, filter_level: Option<LevelFilter>) -> Logger {
    let mut binding = Builder::new();
    let builder = binding
        .parse_filters(ENV_LOGGER_FILTER)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Utc::now().to_rfc3339(),
                record.level(),
                record.module_path().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        });
    if let Some(target) = target {
        builder.target(target);
    }
    if let Some(filter_level) = filter_level {
        builder.filter_level(filter_level);
    }
    builder.build()
}

/* Rust */

/// Configures a global SDK logger that will log to file and will forward log events to
/// an optional application-specific logger.
///
/// If called, it should be called before any SDK methods (for example, before `connect`).
///
/// It must be called only once in the application lifecycle. Alternatively, If the application
/// already uses a globally-registered logger, this method shouldn't be called at all.
///
/// ### Arguments
///
/// - `log_dir`: Location where the the SDK log file will be created. The directory must already exist.
///
/// - `app_logger`: Optional application logger.
///
/// If the application is to use it's own logger, but would also like the SDK to log SDK-specific
/// log output to a file in the configured `log_dir`, then do not register the
/// app-specific logger as a global logger and instead call this method with the app logger as an arg.
///
/// ### Logging Configuration
///
/// Setting `breez_sdk_core::input_parser=debug` will include in the logs the raw payloads received
/// when interacting with JSON endpoints, for example those used during all LNURL workflows.
///
/// ### Errors
///
/// An error is thrown if the log file cannot be created in the working directory.
///
/// An error is thrown if a global logger is already configured.
pub fn init_sdk_logger(
    log_dir: &str,
    app_logger: Option<Box<dyn Log>>,
    filter_level: Option<LevelFilter>,
) -> Result<()> {
    let filter_level = filter_level.unwrap_or(LevelFilter::Trace);

    assert!(
        filter_level <= STATIC_MAX_LEVEL,
        "Should respect STATIC_MAX_LEVEL={:?}, which is done in compile time. level{:?}",
        STATIC_MAX_LEVEL,
        filter_level
    );

    let target_log_file = Box::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{log_dir}/sdk.log"))
            .map_err(|e| anyhow!("Can't create log file: {e}"))?,
    );
    let target = env_logger::Target::Pipe(target_log_file);
    let logger = init_env_logger(Some(target), Some(filter_level));

    let global_logger = GlobalSdkLogger {
        logger,
        log_listener: app_logger,
    };

    set_boxed_logger(Box::new(global_logger))
        .map_err(|e| anyhow!("Failed to set global logger: {e}"))?;
    set_max_level(filter_level);

    Ok(())
}

pub struct GlobalSdkLogger {
    /// SDK internal logger, which logs to file
    logger: Logger,
    /// Optional external log listener, that can receive a stream of log statements
    log_listener: Option<Box<dyn Log>>,
}

impl Log for GlobalSdkLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.logger.log(record);

            if let Some(s) = &self.log_listener.as_ref() {
                if s.enabled(record.metadata()) {
                    s.log(record);
                }
            }
        }
    }

    fn flush(&self) {}
}

/* Binding LevelFilter */

pub fn get_filter_level(filter_level: Option<BindingLevelFilter>) -> LevelFilter {
    match filter_level.unwrap_or(BindingLevelFilter::Trace) {
        BindingLevelFilter::Off => LevelFilter::Off,
        BindingLevelFilter::Error => LevelFilter::Error,
        BindingLevelFilter::Warn => LevelFilter::Warn,
        BindingLevelFilter::Info => LevelFilter::Info,
        BindingLevelFilter::Debug => LevelFilter::Debug,
        BindingLevelFilter::Trace => LevelFilter::Trace,
    }
}
