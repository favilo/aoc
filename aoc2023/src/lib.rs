#![feature(associated_type_defaults)]
#![feature(associated_type_bounds)]
// #![feature(drain_filter)]
// #![feature(hash_drain_filter)]
use std::{
    fmt::Debug,
    fs::read_to_string,
    path::Path,
    time::{Duration, Instant},
};

use color_eyre::{eyre::Context, Result};
use tracking_allocator::AllocationRegistry;

use crate::utils::download_input;

mod parsers;
mod utils;

pub const YEAR: usize = 2023;

macro_rules! run_days {
    ($day:ident = $id:expr, $($days:ident = $ids:expr),* $(,)?) => {
        pub mod $day;
        $(pub mod $days;)*
        pub fn run(days: Vec<usize>, track: bool) -> Result<Duration> {
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

run_days!(
    day01 = 1,
    day02 = 2,
    day03 = 3,
    day04 = 4,
    day05 = 5,
    day06 = 6,
    day07 = 7,
    day08 = 8,
);

pub trait Runner<Part1 = usize, Part2 = usize>
where
    Part1: Debug,
    Part2: Debug,
{
    type Input<'input>;

    fn run(track: bool) -> Result<Duration> {
        let comment = Self::comment();
        let comment = if comment.is_empty() {
            comment.to_owned()
        } else {
            format!(" : {}", comment)
        };
        log::info!("Day {}{}\n", Self::day(), comment);
        let input_path = format!("input/{}/day{:02}.txt", YEAR, Self::day());
        if !Path::new(&input_path).exists() {
            dotenv::dotenv().wrap_err("loading .env file")?;
            let session = std::env::var("AOCSESSION").wrap_err("looking for AOCSESSION env var")?;
            download_input(Self::day(), YEAR, &session, &input_path)?;
        }
        let input = read_to_string(input_path)?;
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
                fn part1() -> Result<()> {
                    let input = $input;
                    println!("{}", input);
                    let input = Day::get_input(input)?;
                    println!("{:#?}", input);
                    assert_eq!($part1, Day::part1(&input)?);
                    Ok(())
                }

                #[test]
                fn part2() -> Result<()> {
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
                fn part1() -> Result<()> {
                    let input = $input1;
                    println!("{}", input);
                    let input = Day::get_input(input)?;
                    println!("{:#?}", input);
                    assert_eq!($part1, Day::part1(&input)?);
                    Ok(())
                }

                #[test]
                fn part2() -> Result<()> {
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
                use std::fs::read_to_string;

                #[test]
                fn part1() -> Result<()> {
                    let input_path = format!("input/{}/day{:02}.txt", crate::YEAR, Day::day());
                    let input = read_to_string(input_path)?;
                    let input = Day::get_input(&input)?;
                    assert_eq!($part1, Day::part1(&input)?);
                    Ok(())
                }

                #[test]
                fn part2() -> Result<()> {
                    let input_path = format!("input/{}/day{:02}.txt", crate::YEAR, Day::day());
                    let input = read_to_string(input_path)?;
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
