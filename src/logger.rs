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

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "<{}/{}> {}",
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        // .level(log::LevelFilter::Off)
        .level(tracing::log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .chain(fern::log_file(LOG_PATH)?)
        .apply()?;
    Ok(())
}
