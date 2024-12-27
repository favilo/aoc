#![feature(allocator_api)]
#![feature(impl_trait_in_assoc_type)]
#![feature(debug_closure_helpers)]
#![warn(clippy::all)]
//#![warn(clippy::pedantic)]
pub mod collections;
pub mod graph;
pub mod errors;
pub mod macros;
pub mod math;
pub mod parse;
pub mod traits;
pub mod utils;

use std::fmt::Debug;
use std::fs::read_to_string;
use std::time::{Duration, Instant};

use heapless::binary_heap::Max;
use heapless::BinaryHeap;
use miette::{Context, IntoDiagnostic, Result};
use tracking_allocator::AllocationRegistry;

use self::utils::file::{download_input, get_input_path};

pub type Heap = BinaryHeap<StageTime, Max, 125>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stage {
    GetInput,
    Part1,
    Part2,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StageTime {
    pub time: Duration,
    day: usize,
    stage: Stage,
    comment: String,
}

impl PartialOrd for StageTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StageTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

impl StageTime {
    pub fn new<Part1, Part2, D: Runner<Part1, Part2>>(time: Duration, stage: Stage) -> Self
    where
        Part1: Debug,
        Part2: Debug,
    {
        Self {
            time,
            stage,
            day: D::day(),
            comment: D::comment().to_string(),
        }
    }

    pub fn log(&self, level: log::Level) {
        let comment = if self.comment.is_empty() {
            String::new()
        } else {
            format!(" - {}", self.comment)
        };
        log::log!(
            level,
            "Day{:02}/{:8?}  -->  {:?}{}",
            self.day,
            self.stage,
            self.time,
            comment,
        )
    }
}

pub trait Runner<Part1 = usize, Part2 = usize>
where
    Part1: Debug,
    Part2: Debug,
{
    type Input<'input>;

    fn day() -> usize;
    #[must_use]
    fn comment() -> &'static str {
        ""
    }

    fn get_input(_: &str) -> Result<Self::Input<'_>>;
    fn part1(_: &Self::Input<'_>) -> Result<Part1>;
    fn part2(_: &Self::Input<'_>) -> Result<Part2>;
}

pub fn run<R, Part1, Part2>(
    year: usize,
    track: bool,
    heap: &mut BinaryHeap<StageTime, Max, 125>,
) -> Result<Duration>
where
    R: Runner<Part1, Part2>,
    Part1: Debug,
    Part2: Debug,
{
    let comment = R::comment();
    let comment = if comment.is_empty() {
        String::new()
    } else {
        format!(" : {comment}")
    };
    log::info!("Day {}{}\n", R::day(), comment);
    let input_full_path = get_input_path(year, R::day())?;
    if !input_full_path.exists() {
        let session = std::env::var("AOCSESSION")
            .into_diagnostic()
            .wrap_err("looking for AOCSESSION env var")?;
        download_input(R::day(), year, &session, &input_full_path)?;
    }
    let input = read_to_string(input_full_path).map_err(|e| miette::miette!("{e}"))?;
    let now = Instant::now();
    if track {
        AllocationRegistry::enable_tracking();
    }
    let input = R::get_input(&input)?;
    let elapsed_i = now.elapsed();
    heap.push(StageTime::new::<Part1, Part2, R>(
        elapsed_i,
        Stage::GetInput,
    ))
    .map_err(|e| miette::miette!("Too many stages: {e:?}"))?;
    log::info!("Generation took {:?}", elapsed_i);

    let now = Instant::now();
    let output1 = R::part1(&input);
    let elapsed1 = now.elapsed();
    let output1 = output1?;
    log::info!("Part 1 - {:?}", output1);
    heap.push(StageTime::new::<Part1, Part2, R>(elapsed1, Stage::Part1))
        .map_err(|e| miette::miette!("Too many stages: {e:?}"))?;
    log::info!("Took {:?}", elapsed1);

    let now = Instant::now();
    let output2 = R::part2(&input);
    let elapsed2 = now.elapsed();
    let output2 = output2?;
    if track {
        AllocationRegistry::disable_tracking();
    }

    log::info!("Part 2 - {:?}", output2);
    heap.push(StageTime::new::<Part1, Part2, R>(elapsed2, Stage::Part2))
        .map_err(|e| miette::miette!("Too many stages: {e:?}"))?;
    log::info!("Took {:?}\n", elapsed2);
    Ok(elapsed_i + elapsed1 + elapsed2)
}
