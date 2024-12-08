use std::cell::Cell;
use std::ops::{Add, AddAssign, Mul};
use std::rc::Rc;

use aoc_utils::collections::multimap::MultiMap;
use hashbrown::HashSet;
use miette::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

use crate::Runner;

pub struct Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord(isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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

#[derive(Debug, Clone)]
pub struct Grid {
    height: isize,
    width: isize,
    obstacles: HashSet<Coord>,
    guard: Coord,
    dir: Direction,

    visited: MultiMap<Coord, Direction>,
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Mul<isize> for Coord {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Grid {
    fn take_walk(&mut self) -> bool {
        loop {
            let next = self.guard + self.dir.delta();
            if next.0 < 0 || next.0 >= self.height || next.1 < 0 || next.1 >= self.width {
                // Guard left area
                // println!("**** LEFT ****: {next:?}");
                return false;
            }

            if self.obstacles.contains(&next) {
                // Hit obstacle
                // println!("**** BUMP ****");
                // self.visited.insert(self.guard, self.dir.turn());
                self.dir = self.dir.turn();
                continue;
            }

            self.guard = next;
            // if we are in a loop, we need to return true
            if self
                .visited
                .get_all(&self.guard)
                .map(|dirs| dirs.contains(&self.dir))
                .unwrap_or(false)
            {
                return true;
            }
            self.visited.insert(self.guard, self.dir);
        }
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
        let obstacles = input
            .lines()
            .enumerate()
            .flat_map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(c, ch)| (c, r, ch))
                    .filter_map(|(c, r, ch)| {
                        if ch == '^' {
                            guard.set(Coord(r as isize, c as isize));
                        }
                        (ch == '#').then_some(Coord(r as isize, c as isize))
                    })
            })
            .collect();
        let guard = guard.get();
        let mut visited = MultiMap::new();
        visited.insert(guard, Direction::Up);
        Ok(Grid {
            height,
            width,
            obstacles,
            guard,
            dir: Direction::Up,

            visited,
        })
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut grid = input.clone();
        grid.take_walk();

        Ok(grid.visited.keys().count())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut grid = input.clone();
        let start = grid.guard;
        grid.take_walk();
        let visited = grid.visited;

        let new_obstacles =
            visited
                .par_iter()
                .filter(|&(c, _)| *c != start)
                .filter(|&(coord, _)| {
                    let mut new_grid = input.clone();
                    new_grid.obstacles.insert(*coord);
                    new_grid.take_walk()
                });
        Ok(new_obstacles.count())
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
