use aoc_utils::{
    collections::multimap::{MultiMap, OrderedMultiMap},
    graph::four_neighbors_no_filter,
    math::coord::Coord,
};
use hashbrown::HashSet;
use itertools::Itertools;
use miette::Result;
use pathfinding::prelude::ConnectedComponents;

use crate::Runner;

pub struct Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Side {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

impl Side {
    pub fn side(&self, coord: Coord) -> isize {
        match self {
            Side::Top | Side::Bottom => coord.0,
            Side::Left | Side::Right => coord.1,
        }
    }

    pub fn ortho(&self, coord: Coord) -> isize {
        match self {
            Side::Top | Side::Bottom => coord.1,
            Side::Left | Side::Right => coord.0,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Farm {
    shape: Coord,
    plots: MultiMap<char, Coord>,
}

impl Farm {
    pub fn regions(&self) -> impl Iterator<Item = (char, HashSet<Coord>)> + '_ {
        self.plots.iter().flat_map(|(c, coord_set)| {
            let starts = Vec::from_iter(coord_set.iter().copied());
            ConnectedComponents::<Coord, Vec<_>, HashSet<_>, HashSet<_>>::connected_components(
                &starts,
                |&coord: &Coord| {
                    four_neighbors_no_filter::<isize>(coord.into())
                        .map(Coord::from)
                        .filter(|coord| coord_set.contains(coord))
                },
            )
            .into_iter()
            .map(|set| (*c, set))
        })
    }

    pub fn find_fences(&self, region: &HashSet<Coord>) -> OrderedMultiMap<(Side, isize), Coord> {
        region
            .iter()
            .flat_map(|&coord| {
                [Side::Top, Side::Left, Side::Bottom, Side::Right]
                    .iter()
                    .zip(four_neighbors_no_filter::<isize>(coord.into()).map(Coord::from))
                    .filter(|(_, c)| !region.contains(c))
                    .map(move |(side, _c)| {
                        (
                            (
                                *side,
                                match side {
                                    Side::Top | Side::Bottom => coord.0,
                                    Side::Left | Side::Right => coord.1,
                                },
                            ),
                            coord,
                        )
                    })
            })
            .collect()
    }

    fn area(&self, region: &HashSet<Coord>) -> usize {
        region.len()
    }

    fn perimeter(&self, region: &HashSet<Coord>) -> usize {
        region
            .iter()
            .copied()
            .map(|c| {
                four_neighbors_no_filter::<isize>(c.into())
                    .map(Coord::from)
                    .filter(|c| !region.contains(c))
                    .count()
            })
            .sum()
    }

    fn sides(&self, region: &HashSet<Coord>) -> usize {
        let mut fences = self.find_fences(region);
        let sides = fences
            .iter_mut()
            .map(|(key, coords)| {
                coords.sort();
                (key, coords)
            })
            .map(|(key, coords)| {
                let mut chunk = key.0.ortho(coords[0]);
                let mut group = chunk;
                coords
                    .iter()
                    .chunk_by(|coord| {
                        if key.0.ortho(**coord) == chunk {
                            group
                        } else if key.0.ortho(**coord) == chunk + 1 {
                            chunk += 1;
                            group
                        } else {
                            chunk = key.0.ortho(**coord);
                            group = chunk;
                            group
                        }
                    })
                    .into_iter()
                    .count()
            })
            .sum();
        sides
    }
}

impl Runner for Day {
    type Input<'input> = Farm;

    fn day() -> usize {
        12
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let plots = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(j, c)| (c, Coord(i as isize, j as isize)))
            })
            .collect::<MultiMap<_, _>>();
        Ok(Farm {
            shape: Coord(height as isize, width as isize),
            plots,
        })
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .regions()
            .map(|(_, region)| input.area(&region) * input.perimeter(&region))
            .sum::<usize>())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .regions()
            .map(|(_, region)| input.area(&region) * input.sides(&region))
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                AAAA\n\
                BBCD\n\
                BBCC\n\
                EEEC\n\
            ";
            part1 = 140;
            part2 = 80;
    }

    sample_case! {
        sample2 =>
            input = "\
              OOOOO\n\
              OXOXO\n\
              OOOOO\n\
              OXOXO\n\
              OOOOO\n\
            ";
            part1 = 772;
            part2 = 436;
    }

    sample_case! {
        sample3 =>
            input = "\
                RRRRIICCFF\n\
                RRRRIICCCF\n\
                VVRRRCCFFF\n\
                VVRCCCJFFF\n\
                VVVVCJJCFE\n\
                VVIVCCJJEE\n\
                VVIIICJJEE\n\
                MIIIIIJJEE\n\
                MIIISIJEEE\n\
                MMMISSJEEE\n\
            ";
            part1 = 1930;
            part2 = 1206;
    }

    sample_case! {
        sample4 =>
            input = "\
                EEEEE\n\
                EXXXX\n\
                EEEEE\n\
                EXXXX\n\
                EEEEE\n\
            ";
            part1 = 692;
            part2 = 236;
    }

    prod_case! {
        part1 = 1375574;
        part2 = 830566;
    }
}
