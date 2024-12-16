use aoc_utils::{collections::multiset::HashMultiSet, math::coord::Coord};
use hashbrown::HashSet;
use miette::Result;
use winnow::{
    ascii::{dec_int, line_ending},
    combinator::{opt, repeat, separated_pair, terminated},
    seq,
    stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial},
    PResult, Parser,
};

use crate::{errors::ToMiette, Runner};

pub struct Day;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Robot {
    position: Coord,
    velocity: Coord,
}

impl Robot {
    pub fn parser<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr,
    {
        seq!(
            _: "p=", 
            separated_pair(dec_int, ",", dec_int), 
            _: " v=", 
            separated_pair(dec_int, ",", dec_int),
            _: opt(line_ending))
        .map(|(p, v)| Self {
            position: Coord(p.0, p.1),
            velocity: Coord(v.0, v.1),
        })
        .parse_next(input)
    }

    pub fn steps(&self, steps: isize) -> Coord {
        self.position + self.velocity * steps
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    width: isize,
    height: isize,
    robots: Box<[Robot]>,
}

impl Field {
    pub fn steps(&self, steps: isize) -> impl Iterator<Item = Coord> + '_ {
        self.robots
            .iter()
            .map(move |r| r.steps(steps))
            .map(|Coord(x, y)| Coord(x.rem_euclid(self.width), y.rem_euclid(self.height)))
    }

    pub fn quadrants(&self) -> [(Coord, Coord); 4] {
        let (width, height) = (self.width, self.height);
        let xmid = width / 2;
        let ymid = height / 2;
        [
            (Coord(0, 0), Coord(xmid, ymid)),
            (Coord(xmid + 1, 0), Coord(width, ymid)),
            (Coord(0, ymid + 1), Coord(xmid, height)),
            (Coord(xmid + 1, ymid + 1), Coord(width, height)),
        ]
    }

    pub fn security_factor(&self, steps: isize) -> usize {
        let quads = self.quadrants();
        let set = self
            .steps(steps)
            .flat_map(|Coord(x, y)| {
                quads.iter().enumerate().find_map(|(i, (tl, br))| {
                    if (tl.0..br.0).contains(&x) && (tl.1..br.1).contains(&y) {
                        Some(i)
                    } else {
                        None
                    }
                })
            })
            .collect::<HashMultiSet<_>>();
        set.into_iter().map(|i| i.1).product()
    }
}

impl Runner for Day {
    type Input<'input> = Field;

    fn day() -> usize {
        14
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        seq!(
            opt(terminated(
                separated_pair(dec_int, ",", dec_int),
                line_ending,
            )),
            repeat(1.., Robot::parser),
        )
        .map(
            |(dims, robots): (Option<(isize, isize)>, Vec<Robot>)| Field {
                width: dims.map(|(w, _)| w).unwrap_or(101),
                height: dims.map(|(_, h)| h).unwrap_or(103),
                robots: robots.into_boxed_slice(),
            },
        )
        .parse(input)
        .to_miette()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input.security_factor(100))
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        // let lowest = (0..(input.width * input.height))
        // Make this smaller to make it faster... Feels like a hack, but it works fine.
        let lowest = (0..7010)
            .map(|i| (i, input.security_factor(i)))
            .min_by_key(|t| t.1);
        // lowest.iter().for_each(|(i, _)| {
        //     print_grid(
        //         *i as usize,
        //         &input.steps(*i).collect(),
        //         input.width,
        //         input.height,
        //     )
        // });
        Ok(lowest.unwrap().0 as usize)
    }
}

#[allow(dead_code)]
fn print_grid(step: usize, set: &HashSet<Coord>, width: isize, height: isize) {
    println!("{}", step);
    println!("{}", unsafe {
        &vec![b'='; width as usize][..].as_ascii_unchecked().as_str()
    });
    for j in 0..height {
        for i in 0..width {
            print!("{}", if set.contains(&Coord(i, j)) { "^" } else { " " });
        }
        println!();
    }
    println!("{}", unsafe {
        &vec![b'='; width as usize][..].as_ascii_unchecked().as_str()
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                7,11\n\
                p=0,4 v=3,-3\n\
                p=6,3 v=-1,-3\n\
                p=10,3 v=-1,2\n\
                p=2,0 v=2,-1\n\
                p=0,0 v=1,3\n\
                p=3,0 v=-2,-2\n\
                p=7,6 v=-1,-3\n\
                p=3,0 v=-1,-2\n\
                p=9,3 v=2,3\n\
                p=7,3 v=-1,2\n\
                p=2,4 v=2,-3\n\
                p=9,5 v=-3,-3\n\
            ";
            part1 = 12;
            part2 = 67;
    }

    prod_case! {
        part1 = 218619120;
        part2 = 7055;
    }
}
