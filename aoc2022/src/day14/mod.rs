use std::{
    collections::HashSet,
    ops::{Add, AddAssign},
};

use color_eyre::Result;
use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::u64, combinator::map, error::VerboseError,
    multi::separated_list1, sequence::separated_pair, IResult,
};

use crate::Runner;

pub struct Day;

type ParseResult<'a, T> = IResult<&'a [u8], T, VerboseError<&'a [u8]>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    Falling,
    Stuck,
    Exited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NextStep {
    coord: Coord,
    status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(i64, i64);

impl Coord {
    fn neighbors(self) -> [Coord; 3] {
        [self + Coord(0, 1), self + Coord(-1, 1), self + Coord(1, 1)]
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Cell {
    #[default]
    Rock,
    Sand,
}

fn rock_point(input: &[u8]) -> ParseResult<Coord> {
    map(separated_pair(u64, tag(","), u64), |(c, r)| {
        Coord(c.try_into().unwrap(), r.try_into().unwrap())
    })(input)
}

fn rock_curve(input: &[u8]) -> ParseResult<Vec<Coord>> {
    let (input, points) = separated_list1(tag(" -> "), rock_point)(input)?;
    let points = points
        .into_iter()
        .tuple_windows()
        .flat_map(|(Coord(c0, r0), Coord(c1, r1))| {
            if c0 == c1 {
                // vertical
                (r0.min(r1)..=r0.max(r1))
                    .into_iter()
                    .map(|r| Coord(c0, r))
                    .collect_vec()
            } else if r0 == r1 {
                (c0.min(c1)..=c0.max(c1))
                    .into_iter()
                    .map(|c| Coord(c, r0))
                    .collect_vec()
            } else {
                unreachable!()
            }
        })
        .collect();
    Ok((input, points))
}

fn parse_map(input: &str) -> Result<(HashSet<Coord>, i64)> {
    let map = input
        .lines()
        .map(str::as_bytes)
        .flat_map(|line| rock_curve(line).unwrap().1)
        .collect::<HashSet<_>>();
    let highest = map.iter().map(|c| c.1).max().unwrap();
    Ok((map, highest))
}

fn run_grain(map: &mut HashSet<Coord>, bottom: i64, has_floor: bool) -> usize {
    let start = Coord(500, 0);
    let stone_count = map.len();
    let mut q = vec![];

    q.push(start);
    while let Some(&c @ Coord(_, y)) = q.last() {
        let found_space = y <= bottom
            && c.neighbors()
                .into_iter()
                .filter(|c| !map.contains(c))
                .any(|c: Coord| -> bool {
                    q.push(c);
                    true
                });

        if !found_space {
            map.insert(c);
            q.pop();
        } else if !has_floor && y >= bottom {
            break;
        }
    }

    map.len() - stone_count
}

impl Runner for Day {
    type Input<'input> = (HashSet<Coord>, i64);

    fn day() -> usize {
        14
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        parse_map(input)
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut map = input.0.clone();
        Ok(run_grain(&mut map, input.1, false))
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut map = input.0.clone();
        Ok(run_grain(&mut map, input.1, true))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                498,4 -> 498,6 -> 496,6\n\
                503,4 -> 502,4 -> 502,9 -> 494,9\n";
            part1 = 24;
            part2 = 93;
    }

    prod_case! {
        part1 = 1001;
        part2 = 27976;
    }
}
