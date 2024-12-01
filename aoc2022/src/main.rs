use std::sync::atomic::AtomicBool;

use clap::{ArgAction, Parser};
use color_eyre::Result;
use fern::colors::{Color, ColoredLevelConfig};
use mimalloc::MiMalloc;
use tracking_allocator::{AllocationGroupId, AllocationRegistry, AllocationTracker, Allocator};

#[global_allocator]
static GLOBAL: Allocator<MiMalloc> = Allocator::from_allocator(MiMalloc);

static PANIC_ON_ALLOCATE: AtomicBool = AtomicBool::new(false);

// #[global_allocator]
// static GLOBAL: Allocator<System> = Allocator::system();

struct StdoutTracker;

impl AllocationTracker for StdoutTracker {
    fn allocated(
        &self,
        addr: usize,
        object_size: usize,
        wrapped_size: usize,
        group_id: AllocationGroupId,
    ) {
        log::info!(
            "allocation -> addr=0x{:0x} object_size={} wrapped_size={} group_id={:?}",
            addr,
            object_size,
            wrapped_size,
            group_id
        );
    }

    fn deallocated(
        &self,
        addr: usize,
        object_size: usize,
        wrapped_size: usize,
        source_group_id: AllocationGroupId,
        current_group_id: AllocationGroupId,
    ) {
        if PANIC_ON_ALLOCATE.load(std::sync::atomic::Ordering::SeqCst) {
            panic!("We've allocated!");
        }

        log::debug!(
            "deallocation -> addr=0x{:0x} object_size={} wrapped_size={} source_group_id={:?} current_group_id={:?}",
            addr, object_size, wrapped_size, source_group_id, current_group_id
        );
    }
}

fn setup_logger() -> Result<()> {
    color_eyre::install()?;
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, action=ArgAction::Append)]
    days: Vec<usize>,
    #[arg(short = 't', long = "track")]
    track_allocations: bool,

    #[arg(short = 'p', long = "panic")]
    panic: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let days: Vec<usize> = args.days;
    setup_logger()?;
    AllocationRegistry::set_global_tracker(StdoutTracker)
        .expect("no other global tracker should be set yet");
    if args.panic {
        PANIC_ON_ALLOCATE.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    let time = aoc2022::run(days, args.track_allocations)?;
    log::info!("Total Time: {:?}", time);

    Ok(())
}
