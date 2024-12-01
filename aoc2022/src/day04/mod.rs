use std::ops::RangeInclusive;

use color_eyre::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map,
    multi::many1,
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::{
    utils::{parse_int, RangeIncExt},
    Runner,
};

pub struct Day;

#[derive(Debug)]
pub struct Pair(RangeInclusive<usize>, RangeInclusive<usize>);

fn range(input: &[u8]) -> IResult<&[u8], RangeInclusive<usize>> {
    let (input, (first, second)) =
        separated_pair(map(digit1, parse_int), tag("-"), map(digit1, parse_int))(input)?;
    Ok((input, first..=second))
}

fn range_pair(input: &[u8]) -> IResult<&[u8], Pair> {
    let (input, (first, second)) = separated_pair(range, tag(","), range)(input)?;
    Ok((input, Pair(first, second)))
}

fn pair_vec(input: &[u8]) -> IResult<&[u8], Vec<Pair>> {
    many1(delimited(multispace0, range_pair, multispace0))(input)
}

impl Runner for Day {
    type Input<'input> = Vec<Pair>;

    fn day() -> usize {
        4
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(pair_vec(input.as_bytes()).unwrap().1)
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .iter()
            .filter(|Pair(f, s)| f.inside_or_surrounding(s))
            .count())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input.iter().filter(|Pair(f, s)| f.overlaps(s)).count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
        input = "\
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8";
        part1 = 2;
        part2 = 4;
    }

    prod_case! {
        part1 = 483;
        part2 = 874;
    }
}
