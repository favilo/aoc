use std::collections::BTreeMap;

use color_eyre::Result;
use itertools::Itertools;
use num::Integer;
use winnow::{
    ascii::{alpha1, alphanumeric1, multispace0, multispace1},
    binary::length_data,
    combinator::{alt, delimited, preceded, repeat},
    PResult, Parser,
};

use crate::Runner;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Right,
    Left,
}

impl Direction {
    fn parse(input: &mut &str) -> PResult<Self> {
        alt(("R".map(|_| Direction::Right), "L".map(|_| Direction::Left))).parse_next(input)
    }
}

#[derive(Debug, Clone)]
pub struct Map<'a> {
    directions: Vec<Direction>,
    map: BTreeMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Map<'a> {
    fn parse(input: &mut &'a str) -> PResult<Self> {
        let directions = repeat(1.., Direction::parse).parse_next(input)?;
        let _ = multispace1.parse_next(input)?;
        let mut map = BTreeMap::new();
        let repeat: Vec<_> = repeat(1.., Self::parse_path).parse_next(input)?;
        repeat.into_iter().for_each(|(head, left, right)| {
            map.insert(head, (left, right));
        });
        Ok(Self { directions, map })
    }

    fn parse_path(input: &mut &'a str) -> PResult<(&'a str, &'a str, &'a str)> {
        let head = alphanumeric1.parse_next(input)?;
        let (left, _, right) = delimited(
            (multispace0, "=", multispace0, "("),
            (alphanumeric1, (",", multispace0), alphanumeric1),
            (")", multispace0),
        )
        .parse_next(input)?;

        Ok((head, left, right))
    }
}

pub struct Day;

impl Runner for Day {
    type Input<'input> = Map<'input>;

    fn day() -> usize {
        8
    }

    fn get_input(mut input: &str) -> Result<Self::Input<'_>> {
        Ok(Map::parse(&mut input).unwrap())
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let steps = input.directions.iter().copied().cycle().enumerate();
        let current = "AAA";

        let steps = length_of_cycle(steps, current, input);
        Ok(steps)
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let currents = input
            .map
            .keys()
            .filter(|key| key.ends_with("A"))
            .collect::<Vec<_>>();

        let steps = currents
            .into_iter()
            .map(|current| {
                length_of_cycle(
                    input.directions.iter().cycle().copied().enumerate(),
                    current,
                    input,
                )
            })
            .reduce(|a, b| a.lcm(&b))
            .unwrap();

        Ok(steps)
    }
}

fn length_of_cycle<'a>(
    mut steps: impl Iterator<Item = (usize, Direction)>,
    mut current: &'a str,
    input: &Map<'a>,
) -> usize {
    let steps = loop {
        let (idx, step) = steps.next().unwrap();
        if current.ends_with("Z") {
            break idx;
        }
        let (left, right) = input.map.get(current).unwrap();
        match step {
            Direction::Right => current = right,
            Direction::Left => current = left,
        }
    };
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input1 = "\
                RL\n\
                \n\
                AAA = (BBB, CCC)\n\
                BBB = (DDD, EEE)\n\
                CCC = (ZZZ, GGG)\n\
                DDD = (DDD, DDD)\n\
                EEE = (EEE, EEE)\n\
                GGG = (GGG, GGG)\n\
                ZZZ = (ZZZ, ZZZ)\n\
            ";
            part1 = 2;
            input2 = "\
                LR\n\
                \n\
                11A = (11B, XXX)\n\
                11B = (XXX, 11Z)\n\
                11Z = (11B, XXX)\n\
                22A = (22B, XXX)\n\
                22B = (22C, 22C)\n\
                22C = (22Z, 22Z)\n\
                22Z = (22B, 22B)\n\
                XXX = (XXX, XXX)\n\
                ";
            part2 = 6;
    }

    prod_case! {
        part1 = 1681;
        part2 = 201684;
    }
}
