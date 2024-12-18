#![feature(associated_type_defaults)]
#![feature(binary_heap_into_iter_sorted)]
#![feature(debug_closure_helpers)]
#![feature(ascii_char)]
#![feature(impl_trait_in_assoc_type)]
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
        ($id:ident => preamble = $preamble:expr; input1 = $input1:expr; part1_r = $part1:expr; input2 = $input2:expr; part2_r = $part2:expr;) => {
            mod $id {
                use super::*;

                #[test]
                fn part1() -> miette::Result<()> {
                    let _ = env_logger::try_init();
                    #[allow(clippy::unused_unit)]
                    {
                        $preamble;
                    };
                    let input = $input1;
                    println!("{}", input);
                    let input = Day::get_input(input)?;
                    println!("{:#?}", input);
                    let output = Day::part1(&input);
                    if $part1.is_err() {
                        assert!(output.is_err());
                        assert_eq!($part1.unwrap_err().to_string(), output.unwrap_err().to_string())
                    } else {
                        assert_eq!($part1.unwrap(), output?);
                    }
                    Ok(())
                }

                #[test]
                fn part2() -> miette::Result<()> {
                    let _ = env_logger::try_init();
                    #[allow(clippy::unused_unit)]
                    {
                        $preamble;
                    };
                    let input = $input2;
                    println!("{}", input);
                    let input = Day::get_input(input)?;
                    println!("{:#?}", input);
                    if $part2.is_err() {
                        assert!(Day::part2(&input).is_err());
                    } else {
                        assert_eq!($part2.unwrap(), Day::part2(&input)?);
                    }
                    Ok(())
                }
            }
        };
        ($id:ident => input1 = $input1:expr; part1_e = $part1_e:expr; input2 = $input2:expr; part2_e = $part2_e:expr;) => {
            sample_case! { $id =>  preamble = {()}; input1 = $input1; part1_r = Err::<_, &'static str>($part1_e); input2 = $input2; part2_r = Err::<_, &'static str>($part2_e); }
        };
        ($id:ident => input1 = $input1:expr; part1 = $part1:expr; input2 = $input2:expr; part2 = $part2:expr;) => {
            sample_case! { $id =>  preamble = {()}; input1 = $input1; part1_r = Ok::<_, &'static str>($part1); input2 = $input2; part2_r = Ok::<_, &'static str>($part2); }
        };
        ($id:ident => preamble = $preamble:expr; input = $input:expr; part1 = $part1:expr; part2 = $part2:expr;) => {
            sample_case! { $id =>  preamble = $preamble; input1 = $input; part1_r = Ok::<_, &'static str>($part1); input2 = $input; part2_r = Ok::<_, &'static str>($part2); }
        };
        ($id:ident => input = $input:expr; part1 = $part1:expr; part2 = $part2:expr;) => {
            sample_case! { $id =>  preamble = (); input = $input; part1 = $part1; part2 = $part2; }
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
                    let _ = env_logger::try_init();
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
                    let _ = env_logger::try_init();
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

run_days!(
    day01 = 1,
    day02 = 2,
    day03 = 3,
    day04 = 4,
    day05 = 5,
    day06 = 6,
    day07 = 7,
    day08 = 8,
    day09 = 9,
    day10 = 10,
    day11 = 11,
    day12 = 12,
    day13 = 13,
    day14 = 14,
    day15 = 15,
    day16 = 16,
    day17 = 17,
    // day18 = 18,
    // day19 = 19,
    // day20 = 20,
    // day21 = 21,
    // day22 = 22,
    // day23 = 23,
    // day24 = 24,
    // day25 = 25,
);
