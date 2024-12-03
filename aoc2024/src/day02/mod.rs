use std::iter::{Chain, Skip, Take};

use aoc_utils::collections::HVec;
use itertools::Itertools;
use miette::Result;
use winnow::{
    ascii::{dec_uint, line_ending, space0},
    combinator::{alt, eof, repeat_till, terminated, trace},
    error::StrContext,
    stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial},
    PResult, Parser,
};

use crate::Runner;

pub struct Day;

fn report<S>(input: &mut S) -> PResult<HVec<usize>>
where
    S: Stream + Compare<&'static str> + StreamIsPartial,
    S::Token: Clone + AsChar,
    S::Slice: AsBStr,
{
    trace(
        "report",
        repeat_till(
            0..10,
            terminated(
                dec_uint::<_, usize, _>.context(StrContext::Label("level")),
                space0.context(StrContext::Expected(' '.into())),
            ),
            alt((line_ending, eof)),
        )
        .map(|t| t.0),
    )
    .context(StrContext::Label("Report"))
    .parse_next(input)
}

#[allow(dead_code)]
fn reports<S>(input: &mut S) -> PResult<Vec<HVec<usize>>>
where
    S: Stream + Compare<&'static str> + StreamIsPartial,
    S::Token: Clone + AsChar,
    S::Slice: AsBStr,
{
    trace("reports", repeat_till(0.., report, eof).map(|t| t.0)).parse_next(input)
}

impl Runner for Day {
    type Input<'input> = Vec<HVec<usize>>;

    fn day() -> usize {
        2
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        // reports.parse(Located::new(input)).to_miette()

        Ok(input
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|n| n.parse::<usize>().expect("number"))
                    .collect()
            })
            .collect())
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .iter()
            .filter(|report| is_safe(report.iter().cloned()))
            .count())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .iter()
            .filter(|report| is_safe_with_dampener(report.iter().cloned()))
            .count())
    }
}

fn is_safe<I>(report: I) -> bool
where
    I: Iterator<Item = usize> + DoubleEndedIterator + Clone,
{
    (report.clone().is_sorted() || itertools::rev(report.clone()).is_sorted())
        && report
            .tuple_windows()
            .map(|(a, b)| (a as isize - b as isize).unsigned_abs())
            .all(|x| (1..=3).contains(&x))
}

fn is_safe_with_dampener<Input>(input: Input) -> bool
where
    Input: Iterator<Item = usize> + Clone + ExactSizeIterator + DoubleEndedIterator,
{
    iter_levels_with_one_removed::<Input>(input).any(is_safe)
}

fn iter_levels_with_one_removed<'i, Input>(
    input: Input,
) -> impl Iterator<Item = Chain<Take<Input>, Skip<Input>>> + 'i
where
    Input: Iterator<Item = usize> + ExactSizeIterator + DoubleEndedIterator + Clone + 'i,
{
    (0..input.clone().count()).map(move |i| input.clone().take(i).chain(input.clone().skip(i + 1)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "7 6 4 2 1\n\
                1 2 7 8 9\n\
                9 7 6 2 1\n\
                1 3 2 4 5\n\
                8 6 4 4 1\n\
                1 3 6 7 9\n\
            ";
            part1 = 2;
            part2 = 4;
    }

    prod_case! {
        part1 = 369;
        part2 = 428;
    }
}
