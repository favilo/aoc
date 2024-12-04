#![feature(associated_type_defaults)]
#![warn(clippy::all)]
//#![warn(clippy::pedantic)]
use std::{
    fmt::Debug,
    fs::read_to_string,
    time::{Duration, Instant},
};

use miette::{IntoDiagnostic, Result, WrapErr};
use tracking_allocator::AllocationRegistry;

use aoc_utils::utils::file::{download_input, get_input_path};

mod errors;
mod parsers;

pub const YEAR: usize = 2024;

macro_rules! run_days {
    ($day:ident = $id:expr, $($days:ident = $ids:expr),* $(,)?) => {
        pub mod $day;
        $(pub mod $days;)*
        pub fn run(days: Vec<usize>, track: bool) -> miette::Result<Duration> {
            let mut total_time = Duration::ZERO;
            if days.is_empty() {
                total_time += $day::Day::run(track)?;
                $(total_time += $days::Day::run(track)?;)*
            } else {
                for day in days {
                    total_time += match day {
                        $id => $day::Day::run(track)?,
                        $($ids => $days::Day::run(track)?,)*
                        _ => panic!("Invalid day passed"),
                    }
                }
            }

            Ok(total_time)
        }
    };
}

run_days!(day01 = 1, day02 = 2, day03 = 3, day04 = 4,);

pub trait Runner<Part1 = usize, Part2 = usize>
where
    Part1: Debug,
    Part2: Debug,
{
    type Input<'input>;

    fn run(track: bool) -> Result<Duration> {
        let comment = Self::comment();
        let comment = if comment.is_empty() {
            String::new()
        } else {
            format!(" : {comment}")
        };
        log::info!("Day {}{}\n", Self::day(), comment);
        let input_full_path = get_input_path(YEAR, Self::day())?;
        if !input_full_path.exists() {
            let session = std::env::var("AOCSESSION")
                .into_diagnostic()
                .wrap_err("looking for AOCSESSION env var")?;
            download_input(Self::day(), YEAR, &session, &input_full_path)?;
        }
        let input = read_to_string(input_full_path).map_err(|e| miette::miette!("{e}"))?;
        let now = Instant::now();
        if track {
            AllocationRegistry::enable_tracking();
        }
        let input = Self::get_input(&input)?;
        let elapsed_i = now.elapsed();
        log::info!("Generation took {:?}", elapsed_i);

        let now = Instant::now();
        let output1 = Self::part1(&input);
        let elapsed1 = now.elapsed();
        let output1 = output1?;
        log::info!("Part 1 - {:?}", output1);
        log::info!("Took {:?}", elapsed1);

        let now = Instant::now();
        let output2 = Self::part2(&input);
        let elapsed2 = now.elapsed();
        let output2 = output2?;
        if track {
            AllocationRegistry::disable_tracking();
        }

        log::info!("Part 2 - {:?}", output2);
        log::info!("Took {:?}\n", elapsed2);
        Ok(elapsed_i + elapsed1 + elapsed2)
    }

    fn day() -> usize;
    #[must_use]
    fn comment() -> &'static str {
        ""
    }

    fn get_input(_: &str) -> Result<Self::Input<'_>>;
    fn part1(_: &Self::Input<'_>) -> Result<Part1>;
    fn part2(_: &Self::Input<'_>) -> Result<Part2>;
}

#[cfg(test)]
pub(crate) mod helpers {
    macro_rules! sample_case {
        ($id:ident => input = $input:expr; part1 = $part1:expr; part2 = $part2:expr;) => {
            mod $id {
                use super::*;

                #[test]
                fn part1() -> miette::Result<()> {
                    let input = $input;
                    println!("{}", input);
                    let input = Day::get_input(input)?;
                    println!("{:#?}", input);
                    assert_eq!($part1, Day::part1(&input)?);
                    Ok(())
                }

                #[test]
                fn part2() -> miette::Result<()> {
                    let input = $input;
                    println!("{}", input);
                    let input = Day::get_input(input)?;
                    println!("{:#?}", input);
                    assert_eq!($part2, Day::part2(&input)?);
                    Ok(())
                }
            }
        };
        ($id:ident => input1 = $input1:expr; part1 = $part1:expr; input2 = $input2:expr; part2 = $part2:expr;) => {
            mod $id {
                use super::*;

                #[test]
                fn part1() -> miette::Result<()> {
                    let input = $input1;
                    println!("{}", input);
                    let input = Day::get_input(input)?;
                    println!("{:#?}", input);
                    assert_eq!($part1, Day::part1(&input)?);
                    Ok(())
                }

                #[test]
                fn part2() -> miette::Result<()> {
                    let input = $input2;
                    println!("{}", input);
                    let input = Day::get_input(input)?;
                    println!("{:#?}", input);
                    assert_eq!($part2, Day::part2(&input)?);
                    Ok(())
                }
            }
        };
    }

    macro_rules! prod_case {
        (part1 = $part1:expr; part2 = $part2:expr;) => {
            mod prod {
                use super::*;
                use aoc_utils::utils::file::get_input_path;
                use miette::{IntoDiagnostic, WrapErr};
                use std::fs::read_to_string;

                #[test]
                fn part1() -> miette::Result<()> {
                    let input_path = get_input_path(crate::YEAR, Day::day())?;
                    let input = read_to_string(input_path)
                        .into_diagnostic()
                        .wrap_err("failed to read input")?;
                    let input = Day::get_input(&input)?;
                    assert_eq!($part1, Day::part1(&input)?);
                    Ok(())
                }

                #[test]
                fn part2() -> miette::Result<()> {
                    let input_path = get_input_path(crate::YEAR, Day::day())?;
                    let input = read_to_string(input_path)
                        .into_diagnostic()
                        .wrap_err("failed to read input")?;
                    let input = Day::get_input(&input)?;
                    assert_eq!($part2, Day::part2(&input)?);
                    Ok(())
                }
            }
        };
    }

    pub(crate) use prod_case;
    pub(crate) use sample_case;
}
