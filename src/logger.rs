use fern::{
    Dispatch,
    colors::{Color, ColoredLevelConfig},
};

pub const LOG_PATH: &str = "pvz.log";

pub fn setup_logger() -> Result<(), fern::InitError> {
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
                record.target(),
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
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file(LOG_PATH)?);

    // 合并两个 dispatch
    Dispatch::new()
        .level(tracing::log::LevelFilter::Trace)
        .level_for("minhook", tracing::log::LevelFilter::Off)
        .chain(stdout_dispatch)
        .chain(file_dispatch)
        .apply()?;

    Ok(())
}
