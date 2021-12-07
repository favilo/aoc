use anyhow::Result;
use clap::{App, Arg};
use fern::colors::{Color, ColoredLevelConfig};

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
    let matches = App::new("AOC2021")
        .version("2021")
        .author("Favil Orbedios <favilo@gmail.com>")
        .arg(
            Arg::with_name("days")
                .short("d")
                .long("day")
                .value_name("day")
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();
    let days: Vec<usize> = matches
        .values_of("days")
        .unwrap_or_default()
        .map(|s| s.parse().unwrap())
        .collect();
    setup_logger()?;

    let time = aoc2021::run(days)?;
    log::info!("Total Time: {:?}", time);

    Ok(())
}
