use std::collections::BTreeSet;

use anyhow::Result;

use crate::Runner;

pub struct Day05;

#[allow(dead_code)]
fn row(id: &usize) -> usize {
    id >> 3
}

impl Runner for Day05 {
    type Input = BTreeSet<usize>;
    type Output = usize;

    fn day() -> usize {
        5
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .lines()
            .map(|l| {
                l.chars().rev().enumerate().fold(0, |acc, (i, c)| {
                    if c == 'B' || c == 'R' {
                        acc + (1 << i)
                    } else {
                        acc
                    }
                })
            })
            .collect())
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        Ok(*input.iter().max().unwrap())
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        Ok(input
            .iter()
            .zip((*input.iter().min().unwrap())..)
            .filter_map(|(id, o)| if *id != o { Some(o) } else { None })
            .next()
            .unwrap())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // #[test]
}
