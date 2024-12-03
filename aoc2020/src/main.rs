use anyhow::Result;
use fern::colors::Color;
use fern::colors::ColoredLevelConfig;

fn setup_logger() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let colors = ColoredLevelConfig::new()
                // use builder methods
                .info(Color::Green)
                .warn(Color::Magenta);
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        // .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() -> Result<()> {
    setup_logger()?;
    let time = aoc2020::run()?;
    log::info!("Total Time: {:?}", time);

    Ok(())
}
