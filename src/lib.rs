// #![feature(box_patterns, box_syntax)]
// #![feature(box_syntax)]

use std::{
    fmt::Debug,
    fs::read_to_string,
    time::{Duration, Instant},
};

use anyhow::Result;

pub mod day1;
// pub mod day10;
// pub mod day11;
// pub mod day12;
// pub mod day13;
// pub mod day14;
// pub mod day15;
// pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
pub mod day2;
// pub mod day20;
// pub mod day21;
pub mod day3;
// pub mod day4;
// pub mod day5;
// pub mod day6;
// pub mod day7;
// pub mod day8;
// pub mod day9;

pub fn run() -> Result<Duration> {
    let mut total_time = day1::Day01::run()?;
    total_time += day2::Day02::run()?;
    total_time += day3::Day03::run()?;

    Ok(total_time)
}

pub trait Runner {
    type Input;
    type Output: Debug;

    fn run() -> Result<Duration> {
        let comment = Self::comment();
        let comment = if comment.is_empty() {
            comment.to_owned()
        } else {
            format!(" : {}", comment)
        };
        log::info!("Day {}{}\n", Self::day(), comment);
        let input = read_to_string(format!("input/2021/day{:02}.txt", Self::day()))?;
        let now = Instant::now();
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

        log::info!("Part 2 - {:?}", output2);
        log::info!("Took {:?}\n", elapsed2);
        Ok(elapsed_i + elapsed1 + elapsed2)
    }

    fn day() -> usize;
    fn comment() -> &'static str {
        ""
    }

    fn get_input(_: &str) -> Result<Self::Input>;
    fn part1(_: &Self::Input) -> Result<Self::Output>;
    fn part2(_: &Self::Input) -> Result<Self::Output>;
}
