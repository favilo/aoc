use std::cell::Cell;
use std::rc::Rc;

use aoc_utils::{
    collections::bitset::{BitSet, Dim, Dimension, FromBitSetIndex, ToBitSetIndex},
    math::coord::Coord,
};
use hashbrown::HashSet;
use miette::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::Runner;

pub struct Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<usize> for Direction {
    type Error = ();

    fn try_from(value: usize) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::Up),
            1 => Ok(Direction::Right),
            2 => Ok(Direction::Down),
            3 => Ok(Direction::Left),
            _ => Err(()),
        }
    }
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

impl Direction {
    fn turn(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn delta(self) -> Coord {
        match self {
            Direction::Up => Coord(-1, 0),
            Direction::Right => Coord(0, 1),
            Direction::Down => Coord(1, 0),
            Direction::Left => Coord(0, -1),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pair(Coord, Direction);

impl From<(Coord, Direction)> for Pair {
    fn from(value: (Coord, Direction)) -> Self {
        Self(value.0, value.1)
    }
}

impl ToBitSetIndex for Pair {
    #[inline(always)]
    fn to_bitset_index(&self, dim: &Dim) -> usize {
        let Pair(coord, dir) = self;
        let dim = dim.bounds().expect("invalid bounds");
        coord.0 as usize + coord.1 as usize * dim[0] + *dir as usize * dim[0] * dim[1]
    }
}

impl FromBitSetIndex for Pair {
    #[inline(always)]
    fn from_bitset_index(index: usize, dim: &Dim) -> Self {
        let dim = dim.bounds().expect("invalid bounds");
        let coord = Coord(
            (index % dim[0]).try_into().unwrap(),
            (index / dim[0] % dim[1]).try_into().unwrap(),
        );
        let dir = (index / dim[0] / dim[1]).try_into().unwrap();
        Self(coord, dir)
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    height: isize,
    width: isize,
    obstacles: BitSet<Coord>,
    guard: Coord,
    start: Coord,
    dir: Direction,

    visited: BitSet<Pair>,
}

impl Grid {
    fn take_walk(&mut self, extra_obstacle: Option<Coord>) -> bool {
        loop {
            let next = self.guard + self.dir.delta();
            if next.0 < 0 || next.0 >= self.height || next.1 < 0 || next.1 >= self.width {
                // Guard left area
                // println!("**** LEFT ****: {next:?}");
                return false;
            }

            if self.obstacles.contains(&next) || extra_obstacle.map_or(false, |c| c == next) {
                // Hit obstacle
                // println!("**** BUMP ****");
                // self.visited.insert(self.guard, self.dir.turn());
                self.dir = self.dir.turn();
                continue;
            }

            self.guard = next;
            // if we are in a loop, we need to return true
            if self.visited.contains(&Pair(self.guard, self.dir)) {
                return true;
            }
            self.visited.insert(Pair(self.guard, self.dir));
        }
    }

    fn reset(&mut self) {
        self.guard = self.start;
        self.dir = Direction::Up;
        self.visited.clear();
        self.visited.insert(Pair(self.guard, self.dir));
    }
}

impl Runner for Day {
    type Input<'input> = Grid;

    fn day() -> usize {
        6
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        let height = input.lines().count() as isize;
        let width = input.lines().next().unwrap().len() as isize;
        let guard = Rc::new(Cell::new(Coord(0, 0)));
        let mut obstacles = BitSet::<Coord>::with_bounds(&[height as usize, width as usize]);
        obstacles.extend(input.lines().enumerate().flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(move |(c, ch)| (c, r, ch))
                .filter_map(|(c, r, ch)| {
                    if ch == '^' {
                        guard.set(Coord(r as isize, c as isize));
                    }
                    (ch == '#').then_some(Coord(r as isize, c as isize))
                })
        }));
        let mut visited = BitSet::with_bounds(&[height as usize, width as usize]);
        visited.insert(Pair(guard.get(), Direction::Up));
        Ok(Grid {
            height,
            width,
            obstacles,
            guard: guard.get(),
            start: guard.get(),
            dir: Direction::Up,

            visited,
        })
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut grid = input.clone();
        grid.take_walk(None);

        let coords = grid
            .visited
            .iter()
            .map(|Pair(c, _)| c)
            .collect::<HashSet<_>>();
        Ok(coords.len())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut grid = input.clone();
        let start = grid.guard;
        grid.take_walk(None);

        let coords = grid
            .visited
            .iter()
            .map(|Pair(c, _)| c)
            .collect::<HashSet<_>>();

        Ok(coords
            .par_iter()
            .filter(|&c| *c != start)
            .filter(|&coord| {
                let mut grid = input.clone();
                grid.take_walk(Some(*coord))
            })
            .count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                ....#.....\n\
                .........#\n\
                ..........\n\
                ..#.......\n\
                .......#..\n\
                ..........\n\
                .#..^.....\n\
                ........#.\n\
                #.........\n\
                ......#...\n\
            ";
            part1 = 41;
            part2 = 6;
    }

    prod_case! {
        part1 = 5153;
        part2 = 1711;
    }
}
