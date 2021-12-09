use std::{collections::HashSet, ops::Mul};

use anyhow::Result;
use ndarray::Array2;
use nom::{
    character::complete::{multispace0, one_of},
    combinator::map,
    error::{convert_error, VerboseError},
    multi::many1,
    sequence::terminated,
    Finish, IResult,
};

use crate::{utils::parse_int, Runner};

fn parse_input<'a>(input: &'a str) -> IResult<&'a str, Vec<usize>, VerboseError<&str>> {
    let r = terminated(
        many1(map(one_of("0123456789"), |s| parse_int(&[s as u8]))),
        multispace0,
    )(input);
    if r.is_err() {
        println!("{}", convert_error(input, r.finish().err().unwrap()));
        panic!()
    }
    r
}

pub struct Day;

impl Runner for Day {
    type Input = Array2<usize>;
    type Output = usize;

    fn day() -> usize {
        9
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let v = input
            .lines()
            .map(parse_input)
            .map(Result::unwrap)
            .map(|t| t.1)
            .map(Vec::into_iter)
            .flatten()
            .collect::<Vec<_>>();
        Ok(Array2::from_shape_vec((height, width), v).unwrap())
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        Ok(low_points(input).map(|(_, v)| v as usize + 1).sum())
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let mut lows = low_points(input)
            .map(|low| basin_size(input, low.0))
            .collect::<Vec<_>>();
        lows.sort();

        Ok(lows[lows.len() - 3..].into_iter().fold(1, Mul::mul))
    }
}

fn basin_size(array: &Array2<usize>, low: (usize, usize)) -> usize {
    let mut visited = HashSet::new();
    let mut stack = vec![low];
    while !stack.is_empty() {
        let this = stack.pop().unwrap();
        if visited.contains(&this) || array[this] == 9 {
            continue;
        }
        visited.insert(this);
        neighbors(this)
            .filter(|idx| array.get(*idx).is_some())
            .for_each(|idx| stack.push(idx));
    }

    visited.len()
}

fn low_points<'a>(input: &'a Array2<usize>) -> impl Iterator<Item = ((usize, usize), usize)> + 'a {
    input
        .indexed_iter()
        .map(|(idx, v)| (idx, v, neighbors(idx)))
        .map(|(idx, v, neigh)| {
            (
                idx,
                v,
                neigh.map(|n| input.get(n)).flatten().collect::<Vec<_>>(),
            )
        })
        .filter(|(_, v, n)| n.into_iter().all(|o| v < o))
        .map(|v| (v.0, *v.1))
}

fn neighbors(idx: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    [
        (idx.0 as isize - 1, idx.1 as isize),
        (idx.0 as isize, idx.1 as isize - 1),
        (idx.0 as isize + 1, idx.1 as isize),
        (idx.0 as isize, idx.1 as isize + 1),
    ]
    .into_iter()
    .filter(|&(x, y)| x >= 0 && y >= 0)
    .map(|(x, y)| (x as usize, y as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

        let input = Day::get_input(input)?;

        println!("{:?}", input);
        assert_eq!(15, Day::part1(&input)?);
        assert_eq!(1134, Day::part2(&input)?);
        Ok(())
    }
}
