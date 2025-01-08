use std::ops::ControlFlow;

use aoc_utils::{errors::ToMiette, math::coord::Coord};
use itertools::iproduct;
use miette::Result;

use aoc_utils::Runner;
use winnow::{
    ascii::{dec_uint, multispace0},
    combinator::{alt, separated},
    seq,
    stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial},
    PResult, Parser,
};

pub struct Day;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    #[default]
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub fn is_vertical(self) -> bool {
        matches!(self, Self::Up | Self::Down)
    }

    pub fn to_unit_coord(self) -> Coord {
        match self {
            Self::Right => Coord(1, 0),
            Self::Left => Coord(-1, 0),
            Self::Up => Coord(0, 1),
            Self::Down => Coord(0, -1),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Segment {
    point: Coord,
    dir: Direction,
    length: usize,
}

impl Segment {
    pub fn segment<S>(input: &mut S) -> PResult<(Direction, usize)>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr,
    {
        (
            alt((
                "R".map(|_| Direction::Right),
                "L".map(|_| Direction::Left),
                "D".map(|_| Direction::Down),
                "U".map(|_| Direction::Up),
            )),
            dec_uint,
        )
            .parse_next(input)
    }

    pub fn start(self) -> Coord {
        self.point
    }

    pub fn end(self) -> Coord {
        self.point + self.dir.to_unit_coord() * self.length
    }

    pub fn contains(&self, c: Coord) -> bool {
        if self.dir.is_vertical() {
            c.0 == self.point.0 && (self.start().1..=self.end().1).contains(&c.1)
        } else {
            c.1 == self.point.1 && (self.start().0..=self.end().0).contains(&c.0)
        }
    }

    pub fn other(&self, c: Coord) -> Coord {
        if c == self.start() {
            self.end()
        } else {
            self.start()
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        assert_ne!(self.dir, other.dir);
        if self.dir.is_vertical() {
            (self.start().1..=self.end().1).contains(&other.start().1)
                && (other.start().0..=other.end().0).contains(&self.start().0)
        } else {
            (self.start().0..=self.end().0).contains(&other.start().0)
                && (other.start().1..=other.end().1).contains(&self.start().1)
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Wire {
    segments: Vec<Segment>,
}

impl Wire {
    pub fn parser<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr,
    {
        separated(1.., Segment::segment, ",")
            .map(|segments: Vec<_>| {
                let mut last_point = Coord::default();
                let segments = segments
                    .into_iter()
                    .map(|(dir, length)| {
                        let new_dir = match dir {
                            Direction::Right | Direction::Left => Direction::Right,
                            Direction::Up | Direction::Down => Direction::Up,
                        };
                        let point = last_point;
                        last_point += dir.to_unit_coord() * length;
                        let point = match dir {
                            Direction::Right | Direction::Up => point,
                            Direction::Left | Direction::Down => last_point,
                        };
                        Segment {
                            dir: new_dir,
                            length,
                            point,
                        }
                    })
                    .collect();
                Wire { segments }
            })
            .parse_next(input)
    }

    pub fn horiz_segments(&self) -> Vec<Segment> {
        self.segments
            .iter()
            .copied()
            .filter(|&s| !s.dir.is_vertical())
            .collect()
    }

    pub fn vertical_segments(&self) -> Vec<Segment> {
        self.segments
            .iter()
            .copied()
            .filter(|&s| s.dir.is_vertical())
            .collect()
    }

    fn steps_to(&self, c: Coord) -> Option<usize> {
        self.segments
            .iter()
            .try_fold((0, Coord(0, 0)), |(dist, last_point), segment| {
                if segment.contains(c) {
                    return ControlFlow::Break(dist + (c - last_point).magnitude());
                }
                ControlFlow::Continue((dist + segment.length, segment.other(last_point)))
            })
            .break_value()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Panel {
    wires: [Wire; 2],
    horizontals: [Vec<Segment>; 2],
    verticals: [Vec<Segment>; 2],
}

impl Panel {
    pub fn parser<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr,
    {
        seq! { Wire::parser, _: multispace0, Wire::parser, _: multispace0 }
            .map(|(a, b)| {
                let starts = [a.horiz_segments(), b.horiz_segments()];
                let verts = [a.vertical_segments(), b.vertical_segments()];
                let wires = [a, b];

                Self {
                    wires,
                    horizontals: starts,
                    verticals: verts,
                }
            })
            .parse_next(input)
    }

    pub fn intersections(&self) -> impl Iterator<Item = Coord> + '_ {
        Self::intersection_iter(self.horizontals[0].iter(), self.verticals[1].iter()).chain(
            Self::intersection_iter(self.horizontals[1].iter(), self.verticals[0].iter()),
        )
    }

    fn intersection_iter<'a>(
        horizontals: impl Iterator<Item = &'a Segment> + Clone + 'a,
        verticals: impl Iterator<Item = &'a Segment> + Clone + 'a,
    ) -> impl Iterator<Item = Coord> + 'a {
        iproduct!(horizontals, verticals).filter_map(|(horizontal, vertical)| {
            horizontal
                .intersects(vertical)
                .then_some(Coord(vertical.start().0, horizontal.start().1))
        })
    }
}

impl Runner for Day {
    type Input<'input> = Panel;

    #[rustfmt::skip]
    fn day() -> usize {
        3
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Panel::parser.parse(input).to_miette()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        input
            .intersections()
            .map(Coord::magnitude)
            .filter(|&m| m != 0)
            .min()
            .ok_or_else(|| miette::miette!("No intersections found"))
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        input
            .intersections()
            .filter_map(|c| Some(input.wires[0].steps_to(c)? + input.wires[1].steps_to(c)?))
            .min()
            .ok_or_else(|| miette::miette!("No intersections found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = indoc::indoc! {"
                R8,U5,L5,D3
                U7,R6,D4,L4
            "};
            part1 = 6;
            part2 = 0;
    }
    sample_case! {
        sample2 =>
            input = indoc::indoc! {"
                R75,D30,R83,U83,L12,D49,R71,U7,L72
                U62,R66,U55,R34,D71,R55,D58,R83
            "};
            part1 = 159;
            part2 = 0;
    }
    sample_case! {
        sample3 =>
            input = indoc::indoc! {"
                R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
                U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
            "};
            part1 = 135;
            part2 = 0;
    }

    prod_case! {
        part1 = 627;
        part2 = 13190;
    }
}
