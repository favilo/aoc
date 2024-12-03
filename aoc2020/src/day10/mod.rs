use anyhow::Result;
use itertools::Itertools;
use std::iter::once;
use std::{collections::BTreeSet, str::FromStr};

use crate::Runner;

pub struct Day10;

impl Runner for Day10 {
    type Input = BTreeSet<usize>;
    type Output = usize;

    fn day() -> usize {
        10
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        let mut v: Self::Input = once(0)
            .chain(input.lines().map(usize::from_str).map(Result::unwrap))
            .collect();
        v.insert(v.iter().next_back().unwrap() + 3);
        Ok(v)
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        let (ones, threes) =
            input
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .fold((0, 0), |(acc1, acc3), diff| {
                    if diff == 1 {
                        (acc1 + 1, acc3)
                    } else {
                        (acc1, acc3 + 1)
                    }
                });
        Ok(ones * threes)
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        // I've got a paper where I figure out how many options can fit
        // in each run of Δ = 1.
        // Thank goodness nothing went beyond 4, figuring out 5 took a long
        // time. Though I now have some interesting thoughts about a binary
        // encoding with this.
        let lookup_values = [1usize, 1, 2, 4, 7, 13];
        // first run starts from the beginning
        let mut start = 0;
        let total = input
            .iter()
            .tuple_windows()
            .enumerate()
            .filter(|(_, (&a, &b))| b - a == 3)
            .map(|t| t.0)
            .map(|idx| {
                // two numbers are locked, let's start a new run
                let total = lookup_values[(idx - start)];
                start = idx + 1;
                total
            })
            .product();
        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample2() -> Result<()> {
        let input = "28\n\
                     33\n\
                     18\n\
                     42\n\
                     31\n\
                     14\n\
                     46\n\
                     20\n\
                     48\n\
                     47\n\
                     24\n\
                     23\n\
                     49\n\
                     45\n\
                     19\n\
                     38\n\
                     39\n\
                     11\n\
                     1\n\
                     32\n\
                     25\n\
                     35\n\
                     8\n\
                     17\n\
                     7\n\
                     9\n\
                     4\n\
                     2\n\
                     34\n\
                     10\n\
                     3";

        let input = Day10::get_input(input)?;
        assert_eq!(220, Day10::part1(&input)?);
        assert_eq!(19208, Day10::part2(&input)?);
        Ok(())
    }

    #[test]
    fn sample1() -> Result<()> {
        let input = "16\n\
                     10\n\
                     15\n\
                     5\n\
                     1\n\
                     11\n\
                     7\n\
                     19\n\
                     6\n\
                     12\n\
                     4";

        let input = Day10::get_input(input)?;
        assert_eq!(35, Day10::part1(&input)?);
        assert_eq!(8, Day10::part2(&input)?);
        Ok(())
    }

    #[test]
    fn made_up() -> Result<()> {
        let input = "1\n\
                     2\n\
                     3\n\
                     4\n\
                     5\n\
                     8";

        let input = Day10::get_input(input)?;
        assert_eq!(10, Day10::part1(&input)?);
        assert_eq!(13, Day10::part2(&input)?);
        Ok(())
    }
}
