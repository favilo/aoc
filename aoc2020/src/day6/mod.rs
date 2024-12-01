use std::collections::HashSet;

use anyhow::Result;

use crate::Runner;

pub struct Day06;
pub struct Day06Slow;

impl Runner for Day06Slow {
    type Input = Vec<Vec<HashSet<char>>>;
    type Output = usize;

    fn day() -> usize {
        6
    }

    fn comment() -> &'static str {
        "Slow"
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .split("\n\n")
            .map(|group| {
                group
                    .lines()
                    .map(|l| l.chars().collect::<HashSet<_>>())
                    .collect()
            })
            .collect())
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        Ok(input
            .iter()
            .map(|group| group.iter().flatten().collect::<HashSet<_>>().len())
            .sum())
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        Ok(input
            .iter()
            .map(|group| {
                group
                    .iter()
                    // This looks weird, but it's twice as fast as  `('a'..='z').collect()` as the initial
                    .fold(None, |acc: Option<HashSet<char>>, set| {
                        acc.map(|a| a.intersection(&set).copied().collect())
                            .or_else(|| Some(set.clone()))
                    })
                    .unwrap()
                    .len()
            })
            .sum())
    }
}

impl Runner for Day06 {
    type Input = Vec<Vec<u32>>;
    type Output = usize;

    fn day() -> usize {
        6
    }

    fn comment() -> &'static str {
        "Bitpacking"
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .split("\n\n")
            .map(|group| {
                group
                    .lines()
                    .map(|l| {
                        let mut bits = 0;
                        l.chars().for_each(|c| bits |= 1 << (c as u8 - b'a'));
                        bits
                    })
                    .collect()
            })
            .collect())
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        Ok(input
            .iter()
            .map(|group| group.iter().fold(0u32, |acc, a| acc | a))
            .map(|g| g.count_ones() as usize)
            .sum())
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        Ok(input
            .iter()
            .map(|group| {
                group
                    .iter()
                    // This looks weird, but it's twice as fast as  `('a'..='z').collect()` as the initial
                    .fold(u32::MAX, |acc, set| acc & set)
            })
            .map(|g| g.count_ones() as usize)
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample1() -> Result<()> {
        let input = "abc\n\
                     \n\
                     a\n\
                     b\n\
                     c\n\
                     \n\
                     ab\n\
                     ac\n\
                     \n\
                     a\n\
                     a\n\
                     a\n\
                     a\n\
                     \n\
                     b";

        let input = Day06::get_input(input)?;
        assert_eq!(11, Day06::part1(&input)?);
        assert_eq!(6, Day06::part2(&input)?);
        Ok(())
    }
}
