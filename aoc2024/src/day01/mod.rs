use std::iter::zip;

use itertools::Itertools;
use miette::Result;
use multiset::HashMultiSet;

use crate::Runner;

pub struct Day;

impl Runner for Day {
    type Input<'input> = (Vec<usize>, Vec<usize>);

    fn day() -> usize {
        1
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(input
            .lines()
            .map(|l| {
                l.split("   ")
                    .map(|n| n.parse::<usize>().unwrap())
                    .next_tuple()
                    .unwrap()
            })
            .collect::<(Vec<usize>, Vec<usize>)>())
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let (mut a, mut b) = input.clone();

        a.sort();
        b.sort();

        Ok(zip(a, b)
            .map(|(a, b)| (a as isize - b as isize).unsigned_abs())
            .sum::<usize>())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let (left, right): (_, HashMultiSet<usize>) = (
            input.0.iter().cloned(),
            HashMultiSet::from_iter(input.1.iter().cloned()),
        );
        Ok(left.map(|v| v * right.count_of(&v)).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "3   4\n\
                4   3\n\
                2   5\n\
                1   3\n\
                3   9\n\
                3   3";
            part1 = 11;
            part2 = 31;
    }

    prod_case! {
        part1 = 1970720;
        part2 = 17191599;
    }
}
