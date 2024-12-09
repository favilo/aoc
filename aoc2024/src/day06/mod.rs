use std::cell::Cell;
use std::rc::Rc;

use aoc_utils::collections::multimap::MultiMap;
use aoc_utils::math::coord::Coord;
use hashbrown::HashSet;
use miette::Result;

use crate::Runner;

pub struct Day;

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
    start: Coord,
    dir: Direction,

    visited: MultiMap<Coord, Direction>,
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

    fn reset(&mut self) {
        self.guard = self.start;
        self.dir = Direction::Up;
        self.visited.clear();
        self.visited.insert(self.guard, self.dir);
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
            start: guard,
            dir: Direction::Up,

            visited,
        })
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut grid = input.clone();
        grid.take_walk(None);

        Ok(grid.visited.keys().count())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut grid = input.clone();
        let start = grid.guard;
        grid.take_walk(None);
        let visited = grid.visited.clone();

        let new_obstacles = visited
            .iter()
            .filter(|&(c, _)| *c != start)
            .filter(|&(coord, _)| {
                grid.reset();
                grid.take_walk(Some(*coord))
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
