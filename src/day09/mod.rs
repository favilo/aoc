use std::{
    collections::{BinaryHeap, HashSet},
    ops::Mul,
};

use anyhow::Result;
use itertools::Itertools;
use ndarray::Array2;

use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    utils::{four_neighbors, single_digit_line},
    Runner,
};

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
        let mut v = input
            .lines()
            .map(str::as_bytes)
            .map(single_digit_line)
            .map(Result::unwrap)
            .map(|t| t.1)
            .map(Vec::into_iter)
            .flatten();
        Ok(Array2::from_shape_fn((height, width), |_| {
            v.next().unwrap()
        }))
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        Ok(low_points(input)
            // .par_bridge()
            .map(|(_, v)| v as usize + 1)
            .sum())
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let mut lows = low_points(input)
            .par_bridge()
            .map(|low| basin_size(input, low.0))
            .collect::<BinaryHeap<_>>();

        let lows = [
            lows.pop().unwrap(),
            lows.pop().unwrap(),
            lows.pop().unwrap(),
        ];
        Ok(lows.into_iter().fold1(Mul::mul).unwrap())
    }
}

fn basin_size(array: &Array2<usize>, low: (usize, usize)) -> usize {
    let shape = array.shape();
    let mut visited = HashSet::with_capacity(array.len());
    let mut stack = vec![low];
    stack.reserve(array.len());
    while !stack.is_empty() {
        let this = stack.pop().unwrap();
        if visited.contains(&this) || array[this] == 9 {
            continue;
        }
        visited.insert(this);
        four_neighbors(this, (shape[0], shape[1]))
            .filter(|idx| array.get(*idx).is_some())
            .for_each(|idx| stack.push(idx));
    }

    visited.len()
}

fn low_points<'a>(input: &'a Array2<usize>) -> impl Iterator<Item = ((usize, usize), usize)> + 'a {
    let shape = input.shape();
    input
        .indexed_iter()
        .map(|(idx, v)| {
            (
                idx,
                v,
                four_neighbors(idx, (shape[0], shape[1]))
                    .map(|n| input.get(n))
                    .flatten(),
            )
        })
        .filter_map(|(idx, v, mut n)| {
            let all: bool = n.all(|o| -> bool { v < &&o });
            all.then(|| (idx, *v))
        })
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
