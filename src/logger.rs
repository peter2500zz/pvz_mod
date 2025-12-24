use config::load_config;
use fern::{
    Dispatch,
    colors::{Color, ColoredLevelConfig},
};
use tracing::{log::LevelFilter};
use serde::Deserialize;

use crate::{CONFIG};

const DEFAULT_LOG_PATH: &str = "pvz.log";
const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Info;

#[derive(Deserialize)]
struct Config {
    log_path: Option<String>,
    log_level: Option<LogLevel>,
}

#[derive(Debug, Deserialize)]
enum LogLevel {
    /// A level lower than all log levels.
    Off,
    /// Corresponds to the `Error` log level.
    Error,
    /// Corresponds to the `Warn` log level.
    Warn,
    /// Corresponds to the `Info` log level.
    Info,
    /// Corresponds to the `Debug` log level.
    Debug,
    /// Corresponds to the `Trace` log level.
    Trace,
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    let cfg = load_config::<Config>(CONFIG);

    let colors = ColoredLevelConfig::new()
        .error(Color::BrightRed)
        .warn(Color::BrightYellow)
        .info(Color::BrightGreen)
        .debug(Color::BrightBlue)
        .trace(Color::BrightMagenta);

    // 带颜色的终端输出
    let stdout_dispatch = Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "<{}/{}> {}",
                record.target().replace("rumia::pvz", "pvz"),
                colors.color(record.level()),
                message
            ))
        })
        .chain(std::io::stdout());

    // 不带颜色的文件输出
    let file_dispatch = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "<{}/{}> {}",
                record.target().replace("rumia::pvz", "pvz"),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file(cfg.log_path.unwrap_or(DEFAULT_LOG_PATH.to_string()))?);

    // 合并两个 dispatch
    Dispatch::new()
        .level(match cfg.log_level.unwrap_or(DEFAULT_LOG_LEVEL) {
            LogLevel::Off => LevelFilter::Off,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        })
        .level_for("minhook", tracing::log::LevelFilter::Off)
        .chain(stdout_dispatch)
        .chain(file_dispatch)
        .apply()?;


    Ok(())
}
