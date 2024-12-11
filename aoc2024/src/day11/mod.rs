use std::iter::once;

use aoc_utils::collections::multiset::HashMultiSet;
use aoc_utils::parse::parse_int;
use miette::Result;

use crate::Runner;

pub struct Day;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Machine {
    /// The stones
    stones: HashMultiSet<usize>,
}

impl Machine {
    pub fn step(&mut self) -> usize {
        let stones = std::mem::take(&mut self.stones);
        self.stones.extend(
            stones
                .into_iter()
                .flat_map(|(s, count)| apply_rules(s).map(move |v| (v, count))),
        );

        self.stones.len()
    }
}

fn apply_rules(stone: usize) -> Box<dyn Iterator<Item = usize>> {
    match stone {
        0 => Box::new(once(1)),
        n if (n.ilog10() + 1) % 2 == 0 => {
            let place = (n.ilog10() + 1) / 2;
            let first = n / (10usize.pow(place));
            let second = n % (10usize.pow(place));
            Box::new([first, second].into_iter())
        }
        n => Box::new(once(n * 2024)),
    }
}

impl Runner for Day {
    type Input<'input> = Machine;

    fn day() -> usize {
        11
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(Machine {
            stones: input
                .split_whitespace()
                .map(str::as_bytes)
                .map(parse_int)
                .collect(),
        })
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut machine = input.clone();
        (0..25)
            .map(|_| machine.step())
            .last()
            .ok_or(miette::miette!("No steps"))
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut machine = input.clone();
        Ok((0..75)
            .map(|i| (i, machine.step()))
            // .inspect(|l| log::info!("{:?}", l))
            .last()
            .ok_or(miette::miette!("No steps"))?
            .1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "125 17";
            part1 = 55312;
            part2 = 65601038650482;
    }

    prod_case! {
        part1 = 217812;
        part2 = 259112729857522;
    }
}
