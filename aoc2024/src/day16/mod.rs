use std::{cell::RefCell, rc::Rc, str::FromStr};

use aoc_utils::math::coord::Coord;
use hashbrown::{HashMap, HashSet};
use miette::{miette, Result};
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

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Cell {
    #[default]
    Empty,
    Wall,
    Start,
    End,
    Path(Facing),
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("."),
            Self::Wall => f.write_str("#"),
            Self::Start => f.write_str("S"),
            Self::End => f.write_str("E"),
            Self::Path(facing) => f.write_fmt(format_args!("{:?}", facing)),
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

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Facing {
    North,
    South,
    #[default]
    East,
    West,
}

impl std::fmt::Debug for Facing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::North => f.write_str("^"),
            Self::South => f.write_str("v"),
            Self::East => f.write_str(">"),
            Self::West => f.write_str("<"),
        }
    }
}

impl Facing {
    pub fn to_delta(self) -> Coord {
        match self {
            Self::North => Coord(0, -1),
            Self::South => Coord(0, 1),
            Self::East => Coord(1, 0),
            Self::West => Coord(-1, 0),
        }
    }

    pub fn turn_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }

    pub fn turn_left(self) -> Self {
        self.turn_right().turn_right().turn_right()
    }

    pub fn from_delta(delta: Coord) -> Option<Self> {
        Some(match delta {
            Coord(0, -1) => Self::North,
            Coord(0, 1) => Self::South,
            Coord(1, 0) => Self::East,
            Coord(-1, 0) => Self::West,
            _ => None?,
        })
    }

    pub fn num_turns(self, other: Self) -> usize {
        let diff = self.to_delta() + other.to_delta();
        match diff.abs() {
            Coord(0, 0) => 2,
            Coord(0, 2) | Coord(2, 0) => 0,
            Coord(1, 1) => 1,
            _ => panic!("How did we get here?"),
        }
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Maze {
    width: usize,
    height: usize,
    map: HashMap<Coord, Cell>,
    facing: Facing,
    start: Coord,
    end: Coord,
}

impl std::fmt::Debug for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Warehouse")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("start", &self.start)
            .field("end", &self.end)
            .field("facing", &self.facing)
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
                                        _ => panic!("Path should only be for debug"),
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
                    ..Self::default()
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

    pub fn neighbors(&self, cf: CoordFacing) -> impl Iterator<Item = (CoordFacing, usize)> + '_ {
        cf.neighbors(&self.map).collect::<Vec<_>>().into_iter()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CoordFacing(Coord, Facing);

impl CoordFacing {
    pub fn neighbors<'this>(
        &'this self,
        map: &'this HashMap<Coord, Cell>,
    ) -> impl Iterator<Item = (CoordFacing, usize)> + 'this {
        log::debug!("Neighbors of {self:?}");
        [
            (CoordFacing(self.0 + self.1.to_delta(), self.1), 1),
            (CoordFacing(self.0, self.1.turn_right()), 1000),
            (CoordFacing(self.0, self.1.turn_left()), 1000),
        ]
        .into_iter()
        .filter(|&(c, _)| map.contains_key(&c.0))
        .filter(|&(c, _)| map.get(&c.0) != Some(&Cell::Wall))
        .inspect(|(c, s)| log::debug!("{c:?}: {s}"))
    }

    pub fn score(self, next: Coord) -> Option<usize> {
        let delta = next - self.0;
        let facing = Facing::from_delta(delta)?;
        if delta.sq_magnitude() > 1 {
            return None;
        }
        Some(match self.1.num_turns(facing) {
            0 => 1,
            1 => 1000,
            2 => 2000,
            _ => None?,
        })
    }
}

impl Runner for Day {
    type Input<'input> = Maze;

    fn day() -> usize {
        16
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Maze::parser.parse(input).to_miette()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let (v, score) = pathfinding::directed::astar::astar(
            &CoordFacing(input.start, input.facing),
            |cf| input.neighbors(*cf),
            |_cf| 1,
            |cf| input.end == cf.0,
        )
        .ok_or(miette!("No path found"))?;
        log::debug!("{v:?}");
        let mut maze = input.clone();
        for CoordFacing(c, f) in v {
            maze.map.insert(c, Cell::Path(f));
        }
        log::debug!("{maze:?}");
        Ok(score)
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let (solns, _score) = pathfinding::directed::astar::astar_bag(
            &CoordFacing(input.start, input.facing),
            |cf| input.neighbors(*cf),
            |_cf| 1,
            |cf| input.end == cf.0,
        )
        .ok_or(miette!("No path found"))?;
        let any_path_coords = solns
            .flat_map(|cfs| cfs.into_iter().map(|CoordFacing(c, _)| c))
            .collect::<HashSet<_>>();
        log::debug!("{any_path_coords:?}");
        Ok(any_path_coords.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                ###############\n\
                #.......#....E#\n\
                #.#.###.#.###.#\n\
                #.....#.#...#.#\n\
                #.###.#####.#.#\n\
                #.#.#.......#.#\n\
                #.#.#####.###.#\n\
                #...........#.#\n\
                ###.#.#####.#.#\n\
                #...#.....#.#.#\n\
                #.#.#.###.#.#.#\n\
                #.....#...#.#.#\n\
                #.###.#.#.#.#.#\n\
                #S..#.....#...#\n\
                ###############\n\
            ";
            part1 = 7036;
            part2 = 45;
    }

    prod_case! {
        part1 = 102488;
        part2 = 559;
    }
}
