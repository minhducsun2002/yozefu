//! Logging utilities.

use chrono::Local;
use lib::Error;
use log::{LevelFilter, SetLoggerError};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

/// Returns the log level based on the debug flag.
fn log_level(is_debug: bool) -> LevelFilter {
    match is_debug {
        true => LevelFilter::Debug,
        false => LevelFilter::Info,
    }
}

/// When the user starts the headless mode, it writes logs to `stderr`.
pub(crate) fn init_logging_stderr(is_debug: bool) -> Result<(), SetLoggerError> {
    let level = log_level(is_debug);
    let mut logger = env_logger::builder();
    logger
        .filter_level(level)
        .target(env_logger::Target::Stderr)
        .try_init()
}

/// When the user starts the TUI, it writes logs to a file.
pub(crate) fn init_logging_file(is_debug: bool, path: &PathBuf) -> Result<(), Error> {
    let level = log_level(is_debug);
    let file = OpenOptions::new().append(true).create(true).open(path)?;

    let target = Box::new(file);
    env_logger::builder()
        .target(env_logger::Target::Pipe(target))
        .filter_level(level)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, false),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .try_init()
        .map_err(|e| Error::Error(e.to_string()))
}
