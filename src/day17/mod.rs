use std::ops::RangeInclusive;

use anyhow::Result;
use itertools::{iterate, Itertools};
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, one_of},
    combinator::{map, recognize},
    error::{convert_error, VerboseError},
    multi::many1,
    sequence::{delimited, terminated, tuple},
    Finish,
};

use crate::Runner;

pub struct Day;

impl Runner<isize, isize> for Day {
    type Input = (RangeInclusive<isize>, RangeInclusive<isize>);

    fn day() -> usize {
        17
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        let // (input, (x0, x1, y0, y1))
            r = tuple::<_, _, VerboseError<&str>, _>((
            map(
                delimited(
                    tag("target area: x="),
                    recognize(many1(one_of("-0123456789"))),
                    tag(".."),
                ),
                |s: &str| s.parse().unwrap(),
            ),
            map(
                terminated(recognize(many1(one_of("-0123456789"))), tag(", y=")),
                |s: &str| s.parse().unwrap(),
            ),
            map(
                terminated(recognize(many1(one_of("-0123456789"))), tag("..")),
                |s: &str| s.parse().unwrap(),
            ),
            map(
                terminated(recognize(many1(one_of("-0123456789"))), multispace0),
                |s: &str| s.parse().unwrap(),
            ),
        ))(input);
        if r.is_err() {
            println!("{}", &convert_error(input, r.finish().err().unwrap()));
            panic!("error");
        }
        let (_input, (x0, x1, y0, y1)) = r.unwrap();
        Ok(((x0..=x1), (y0..=y1)))
    }

    fn part1(input: &Self::Input) -> Result<isize> {
        let (xs, ys) = input.clone();
        Ok((0..=*xs.end())
            .into_iter()
            .cartesian_product(-1000..1000)
            .map(|(dx, dy)| {
                // println!("Checking: {:?}", (dx, dy));
                steps((dx, dy))
                    .take_while(|p| {
                        // println!("Checking: {:?}", p);
                        p.not_impossible((xs.clone(), ys.clone()))
                    })
                    .map(|p| p.coord)
                    .fold((0, (0, 0)), |(max, _last), p| (std::cmp::max(max, p.1), p))
            })
            .filter_map(|(max, last)| (xs.contains(&last.0) && ys.contains(&last.1)).then(|| max))
            .max()
            .unwrap())
    }

    fn part2(input: &Self::Input) -> Result<isize> {
        let (xs, ys) = input.clone();
        Ok((0..=*xs.end())
            .into_iter()
            .cartesian_product(-1000..1000)
            .map(|(dx, dy)| {
                // println!("Checking: {:?}", (dx, dy));
                steps((dx, dy))
                    .take_while(|p| {
                        // println!("Checking: {:?}", p);
                        p.not_impossible((xs.clone(), ys.clone()))
                    })
                    .map(|p| p.coord)
                    .fold((0, (0, 0)), |(max, _last), p| (std::cmp::max(max, p.1), p))
            })
            .filter_map(|(max, last)| (xs.contains(&last.0) && ys.contains(&last.1)).then(|| max))
            .count() as isize)
    }
}

#[derive(Debug, Clone, Copy)]
struct Probe {
    coord: (isize, isize),
    delta: (isize, isize),
}

impl Probe {
    fn not_impossible(&self, target: (RangeInclusive<isize>, RangeInclusive<isize>)) -> bool {
        self.coord.0 <= *target.0.end() // x is less than the end of the range
            && self.coord.1 >= *target.1.start() // y is greater than the start, since y is negative...
            && !(self.delta.0 == 0 && self.coord.0 < *target.0.start()) // x hasn't stopped before the target
    }
}

fn steps((x, y): (isize, isize)) -> impl Iterator<Item = Probe> {
    iterate(
        Probe {
            coord: (0, 0),
            delta: (x, y),
        },
        |p: &Probe| {
            let mut p = p.clone();
            p.coord = (p.coord.0 + p.delta.0, p.coord.1 + p.delta.1);
            p.delta.1 -= 1;
            p.delta.0 -= p.delta.0.signum();
            p
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "target area: x=20..30, y=-10..-5";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(45, Day::part1(&input)?);
        assert_eq!(112, Day::part2(&input)?);
        Ok(())
    }
}
