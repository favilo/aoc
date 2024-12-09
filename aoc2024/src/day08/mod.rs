use aoc_utils::{collections::multimap::MultiMap, math::coord::Coord};
use hashbrown::HashSet;
use itertools::Itertools;
use miette::Result;

use crate::Runner;

pub struct Day;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    height: usize,
    width: usize,
    nodes: MultiMap<char, Coord>,
}

pub fn get_antinodes(a: Coord, b: Coord) -> [Coord; 2] {
    let diff = b - a;

    [a - diff, b + diff]
}

pub fn get_resonant_antinodes(
    a: Coord,
    b: Coord,
    limits: (usize, usize),
) -> impl Iterator<Item = Coord> {
    let diff = b - a;
    let make_iter = move |c: Coord, diff: Coord| {
        let (height, width) = limits;
        (1..)
            .map(move |i| c + diff * i)
            .take_while(move |c| c.inside_limits(height, width))
    };

    make_iter(a, diff).chain(make_iter(b, -diff))
}

impl Runner for Day {
    type Input<'input> = Grid;

    fn day() -> usize {
        8
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let nodes = input
            .lines()
            .enumerate()
            .flat_map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, ch)| *ch != '.')
                    .map(move |(c, ch)| (ch, Coord(r as isize, c as isize)))
            })
            .collect();
        Ok(Grid {
            height,
            width,
            nodes,
        })
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let set = input
            .nodes
            .iter()
            .flat_map(|(ch, set)| {
                set.iter()
                    .copied()
                    .tuple_combinations()
                    .flat_map(|(a, b)| get_antinodes(a, b))
                    .map(move |c| (*ch, c))
            })
            .filter(|(_, c)| c.inside_limits(input.height, input.width))
            .map(|(_, c)| c)
            .collect::<HashSet<_>>();

        Ok(set.len())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let set = input
            .nodes
            .iter()
            .flat_map(|(ch, set)| {
                set.iter()
                    .copied()
                    .tuple_combinations()
                    .flat_map(|(a, b)| get_resonant_antinodes(a, b, (input.height, input.width)))
                    .map(move |c| (*ch, c))
            })
            .filter(|(_, c)| c.inside_limits(input.height, input.width))
            .map(|(_, c)| c)
            .collect::<HashSet<_>>();

        Ok(set.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                ............\n\
                ........0...\n\
                .....0......\n\
                .......0....\n\
                ....0.......\n\
                ......A.....\n\
                ............\n\
                ............\n\
                ........A...\n\
                .........A..\n\
                ............\n\
                ............\n\
            ";
            part1 = 14;
            part2 = 34;
    }

    prod_case! {
        part1 = 398;
        part2 = 1333;
    }
}
