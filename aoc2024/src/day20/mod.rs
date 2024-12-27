use std::{
    cell::RefCell,
    rc::Rc,
    str::FromStr,
    sync::atomic::{AtomicUsize, Ordering},
};

use aoc_utils::{graph::four_neighbors, math::coord::Coord};
use hashbrown::HashMap;
use itertools::Itertools;
use miette::Result;
use winnow::{
    ascii::line_ending,
    combinator::{repeat, terminated},
    stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial},
    token::take,
    PResult, Parser,
};

use crate::errors::{Error, ToMiette};
use aoc_utils::Runner;

pub struct Day;

static PART1: AtomicUsize = AtomicUsize::new(100);
static PART2: AtomicUsize = AtomicUsize::new(100);

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Cell {
    #[default]
    Empty,
    Wall,
    Start,
    End,
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("."),
            Self::Wall => f.write_str("#"),
            Self::Start => f.write_str("S"),
            Self::End => f.write_str("E"),
        }
    }
}

impl FromStr for Cell {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Empty),
            "#" => Ok(Self::Wall),
            "S" => Ok(Self::Start),
            "E" => Ok(Self::End),
            _ => Err(Error::InvalidInput(s.to_string())),
        }
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Maze {
    width: usize,
    height: usize,
    map: HashMap<Coord, Cell>,
    start: Coord,
    end: Coord,
}

impl std::fmt::Debug for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Maze")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("start", &self.start)
            .field("end", &self.end)
            .field_with("map", |f| {
                f.write_str("\n")?;
                (0..self.height).try_for_each(|j| {
                    let mut s = String::with_capacity(self.width + 1);
                    (0..self.width).for_each(|i| {
                        s.push_str(&format!(
                            "{:?}",
                            self.map
                                .get(&Coord(i as isize, j as isize))
                                .unwrap_or(&Cell::Empty)
                        ))
                    });
                    s.push('\n');
                    f.write_str(&s)
                })
            })
            .finish()
    }
}

impl Maze {
    pub fn parser<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone + Copy,
        <S as Stream>::Slice: AsBStr + AsRef<str>,
    {
        repeat(1.., Self::line)
            .map(|lines: Vec<Vec<Cell>>| {
                let height = lines.len();
                let width = lines.first().unwrap().len();
                let start = Rc::new(RefCell::new(Coord(0, 0)));
                let end = Rc::new(RefCell::new(Coord(0, 0)));
                let map = lines
                    .into_iter()
                    .enumerate()
                    .flat_map(|(j, line)| {
                        let start = start.clone();
                        let end = end.clone();
                        line.into_iter()
                            .enumerate()
                            .inspect(move |&(i, c)| {
                                if c == Cell::Start {
                                    *start.borrow_mut() = Coord(i as isize, j as isize);
                                }
                                if c == Cell::End {
                                    *end.borrow_mut() = Coord(i as isize, j as isize);
                                }
                            })
                            .map(move |(i, c)| {
                                (
                                    Coord(i as isize, j as isize),
                                    match c {
                                        Cell::Empty | Cell::Start | Cell::End => Cell::Empty,
                                        Cell::Wall => Cell::Wall,
                                        // _ => panic!("Path should only be for debug"),
                                    },
                                )
                            })
                    })
                    .collect();
                let start = *start.borrow();
                let end = *end.borrow();
                Self {
                    width,
                    height,
                    map,
                    start,
                    end,
                }
            })
            .parse_next(input)
    }

    fn line<S>(input: &mut S) -> PResult<Vec<Cell>>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone + Copy,
        <S as Stream>::Slice: AsBStr + AsRef<str>,
    {
        terminated(repeat(1.., Self::cell), line_ending).parse_next(input)
    }

    fn cell<S>(input: &mut S) -> PResult<Cell>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone + Copy,
        <S as Stream>::Slice: AsBStr + AsRef<str>,
    {
        take(1usize)
            .try_map(|c: <S as Stream>::Slice| Cell::from_str(c.as_ref()))
            .parse_next(input)
    }

    pub fn path(&self) -> Option<Vec<Coord>> {
        let path = pathfinding::directed::astar::astar(
            &self.start,
            |&c| {
                four_neighbors(c, (self.width, self.height))
                    .filter(|c| self.map.get(c) != Some(&Cell::Wall))
                    .map(|c| (c, 1))
            },
            |_| 1,
            |&c| c == self.end,
        );
        path.map(|p| p.0)
    }
}

impl Runner for Day {
    type Input<'input> = Maze;

    #[rustfmt::skip]
    fn day() -> usize {
        20
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Maze::parser.parse(input).to_miette()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let path = input.path().ok_or(miette::miette!("No path found"))?;
        let limit = PART1.load(Ordering::SeqCst);
        log::debug!("Path: {path:?}");
        let max_dist = 2;

        Ok(count_cheats(path, max_dist, limit))
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let path = input.path().ok_or(miette::miette!("No path found"))?;
        let limit = PART2.load(Ordering::SeqCst);
        log::debug!("Path: {path:?}");
        let max_dist = 20;

        Ok(count_cheats(path, max_dist, limit))
    }
}

fn count_cheats(path: Vec<Coord>, max_dist: usize, limit: usize) -> usize {
    path.into_iter()
        .enumerate()
        .tuple_combinations()
        .filter_map(|((i, a), (j, b)): ((usize, Coord), (usize, Coord))| {
            let skip_dist = a.manhattan_distance(b);
            log::debug!("{a:?}<->{b:?} dist: {skip_dist}");
            log::debug!(
                "{a:?}<->{b:?} map: {}",
                (i as isize - j as isize).unsigned_abs()
            );
            if skip_dist <= max_dist {
                Some(i.abs_diff(j) - skip_dist)
            } else {
                None
            }
        })
        .filter(|&i| i >= limit)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            preamble = {
                PART1.store(1, Ordering::SeqCst);
                PART2.store(50, Ordering::SeqCst);
            };
            input = indoc::indoc! {"
                ###############
                #...#...#.....#
                #.#.#.#.#.###.#
                #S#...#.#.#...#
                #######.#.#.###
                #######.#.#...#
                #######.#.###.#
                ###..E#...#...#
                ###.#######.###
                #...###...#...#
                #.#####.#.###.#
                #.#...#.#.#...#
                #.#.#.#.#.#.###
                #...#...#...###
                ###############
            "};
            part1 = 44;
            part2 = 285;
    }

    prod_case! {
        part1 = 1389;
        part2 = 1005068;
    }
}
