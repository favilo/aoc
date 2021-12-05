use std::cmp::{max, min};

use anyhow::Result;
use itertools::zip;
use ndarray::Array2;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map, sequence::tuple, IResult,
};

use crate::Runner;

pub type Point = (usize, usize);

pub struct Day;

impl Runner for Day {
    type Input = Vec<String>;
    type Output = usize;

    fn day() -> usize {
        5
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input.lines().map(ToOwned::to_owned).collect())
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        let lines = input
            .iter()
            .map(|l| parse_line(l, false))
            .map(Result::unwrap)
            .map(|t| t.1)
            .collect::<Vec<Vec<Point>>>();
        let (max_x, max_y) = lines
            .iter()
            .flatten()
            .copied()
            .fold((0, 0), |(a0, a1), (x, y)| (max(a0, x), max(a1, y)));
        let mut grid = Array2::<usize>::zeros((max_x + 1, max_y + 1));
        lines.iter().flatten().copied().for_each(|(x, y)| {
            grid[(x, y)] += 1usize;
        });
        Ok(grid.into_iter().filter(|n| *n > 1usize).count())
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let lines = input
            .iter()
            .map(|l| parse_line(l, true))
            .map(Result::unwrap)
            .map(|t| t.1)
            .collect::<Vec<Vec<Point>>>();
        let (max_x, max_y) = lines
            .iter()
            .flatten()
            .copied()
            .fold((0, 0), |(a0, a1), (x, y)| (max(a0, x), max(a1, y)));
        let mut grid = Array2::zeros((max_x + 1, max_y + 1));
        lines.iter().flatten().copied().for_each(|(x, y)| {
            grid[(x, y)] += 1usize;
        });
        Ok(grid.into_iter().filter(|n: &usize| *n > 1usize).count())
    }
}

fn parse_line(input: &str, diag: bool) -> IResult<&str, Vec<Point>> {
    let number = |input| -> IResult<&str, usize> {
        map(digit1, |s| usize::from_str_radix(s, 10).unwrap())(input)
    };
    let (input, (x1, _, y1, _, x2, _, y2)) = tuple((
        number,
        tag(","),
        number,
        tag(" -> "),
        number,
        tag(","),
        number,
    ))(input)?;
    Ok((
        input,
        if x1 == x2 {
            (min(y1, y2)..=max(y1, y2)).map(|y| (x1, y)).collect()
        } else if y1 == y2 {
            (min(x1, x2)..=max(x1, x2)).map(|x| (x, y1)).collect()
        } else {
            if diag {
                let xs: Vec<_> = if x1 > x2 {
                    (x2..=x1).rev().collect()
                } else {
                    (x1..=x2).collect()
                };
                let ys: Vec<_> = if y1 > y2 {
                    (y2..=y1).rev().collect()
                } else {
                    (y1..=y2).collect()
                };
                zip(xs, ys).collect()
            } else {
                vec![]
            }
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(5, Day::part1(&input)?);
        assert_eq!(12, Day::part2(&input)?);
        Ok(())
    }
}
