use color_eyre::Result;
use winnow::{
    ascii::{digit1, multispace0, space1},
    combinator::separated,
    PResult, Parser,
};

fn parse_races(input: &mut &str) -> PResult<Vec<(usize, usize)>> {
    let _ = (multispace0, "Time:", space1).parse_next(input)?;
    let times: Vec<_> =
        separated(1.., digit1.try_map(str::parse::<usize>), space1).parse_next(input)?;
    let _ = (multispace0, "Distance:", space1).parse_next(input)?;
    let distances: Vec<_> =
        separated(1.., digit1.try_map(str::parse::<usize>), space1).parse_next(input)?;

    Ok(times.into_iter().zip(distances).collect())
}

use crate::Runner;

pub struct Day;

impl Runner for Day {
    type Input<'input> = Vec<(usize, usize)>;

    fn day() -> usize {
        6
    }

    fn get_input(mut input: &str) -> Result<Self::Input<'_>> {
        Ok(parse_races(&mut input).expect("failed to parse input"))
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .into_iter()
            .copied()
            .map(|(t, dist)| (0..t).map(|i| (t - i) * i).filter(|&d| d > dist).count())
            .product::<usize>())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let (times, distances): (Vec<_>, Vec<_>) = input.into_iter().copied().unzip();
        let time = times
            .into_iter()
            .map(|t| format!("{t}"))
            .collect::<String>()
            .parse::<usize>()?;
        let distance = distances
            .into_iter()
            .map(|d| format!("{d}"))
            .collect::<String>()
            .parse::<usize>()?;
        let first = (0..time / 2).find(|i| (time - i) * i > distance).unwrap();
        Ok(time - first * 2 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                Time:      7  15   30 \
                Distance:  9  40  200 \
            ";
            part1 = 288;
            part2 = 71503;
    }

    prod_case! {
        part1 = 138915;
        part2 = 27340847;
    }
}
