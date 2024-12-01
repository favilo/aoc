use anyhow::bail;
use anyhow::Result;
use itertools::Itertools;
use std::str::FromStr;

use crate::Runner;

pub struct Day09;

impl Runner for Day09 {
    type Input = Vec<usize>;
    type Output = (usize, usize);

    fn day() -> usize {
        9
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .lines()
            .map(usize::from_str)
            .map(Result::unwrap)
            .collect())
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        get_oddball(input, 26)
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let (idx, target) = get_oddball(input, 26)?;
        get_run(input, idx, target)
    }

    fn run() -> Result<std::time::Duration> {
        let comment = Self::comment();
        let comment = if comment.is_empty() {
            comment.to_owned()
        } else {
            format!(" : {}", comment)
        };
        log::info!("Day {}{}\n", Self::day(), comment);
        let input = std::fs::read_to_string(format!("input/2020/day{}.txt", Self::day()))?;
        let now = std::time::Instant::now();
        let input = Self::get_input(&input)?;
        let elapsed_i = now.elapsed();
        log::info!("Generation took {:?}", elapsed_i);

        let now = std::time::Instant::now();
        let output1 = get_oddball(&input, 26);
        let elapsed1 = now.elapsed();
        let output1 = output1?;
        log::info!("Part 1 - {:?}", output1);
        log::info!("Took {:?}", elapsed1);

        let now = std::time::Instant::now();
        let output2 = get_run(&input, output1.0, output1.1);
        let elapsed2 = now.elapsed();
        let output2 = output2?;

        log::info!("Part 2 - {:?}", output2);
        log::info!("Took {:?}\n", elapsed2);
        Ok(elapsed_i + elapsed1 + elapsed2)
    }

    fn comment() -> &'static str {
        ""
    }
}

pub fn get_oddball(input: &[usize], window: usize) -> Result<(usize, usize)> {
    Ok(input
        .windows(window)
        .enumerate()
        .map(|(i, window)| {
            let check = window[window.len() - 1];
            (
                i,
                check,
                window[..window.len() - 1]
                    .iter()
                    .tuple_combinations()
                    .find(move |(&a, &b)| a + b == check),
            )
        })
        .filter(|c| c.2.is_none())
        .map(|c| (c.0, c.1))
        .next()
        .unwrap())
}

pub fn get_run(input: &[usize], idx: usize, target: usize) -> Result<(usize, usize)> {
    for i in 2..idx {
        let mut answer = input[..idx].windows(i).filter_map(|window| {
            if window.iter().sum::<usize>() == target {
                Some((window.iter().min().unwrap(), window.iter().max().unwrap()))
            } else {
                None
            }
        });

        let answer = answer.next();
        if let Some(answer) = answer {
            return Ok((i, answer.0 + answer.1));
        }
    }
    bail!("Should have been found by now");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "35\n\
                     20\n\
                     15\n\
                     25\n\
                     47\n\
                     40\n\
                     62\n\
                     55\n\
                     65\n\
                     95\n\
                     102\n\
                     117\n\
                     150\n\
                     182\n\
                     127\n\
                     219\n\
                     299\n\
                     277\n\
                     309\n\
                     576";

        let input = Day09::get_input(input)?;
        println!("{:#?}", input);
        let (idx, target) = get_oddball(&input, 6)?;
        assert_eq!(127, target);
        assert_eq!(62, get_run(&input, idx, target)?.1);
        Ok(())
    }
}
