use std::collections::VecDeque;
use std::sync::Arc;

use aoc_utils::collections::bitset::BitSet;
use aoc_utils::graph::four_neighbors;
use aoc_utils::math::coord::Coord;
use aoc_utils::parse::parse_uint;
use miette::Result;
use ndarray::ArcArray2;

use crate::Runner;

pub struct Day;

type Idx = (usize, usize);

/// Iterative Depth-first search with visited set
/// Count the number of paths from start at 0 to end at 9
/// monotonically increasing the value the entire time.
fn dfs_with_visited(array: &ArcArray2<usize>, start: Idx) -> usize {
    let mut visited = BitSet::<Coord>::with_bounds(array.shape());
    let shape = array.shape();
    let shape = (shape[0], shape[1]);

    let mut stack = VecDeque::from([start]);
    let mut count = 0;
    while let Some(current) = stack.pop_back() {
        let value = *array.get(current).unwrap();
        visited.insert(current.into());
        if value == 9 {
            count += 1;
            continue;
        }
        four_neighbors(current, shape)
            .filter(|&idx| array.get(idx).is_some())
            .filter(|&idx| array[idx] == value + 1)
            .for_each(|idx| {
                if !visited.contains(&idx.into()) {
                    stack.push_back(idx);
                }
            });
    }
    count
}

/// Recursive Depth-first search
/// Count the number of paths from start at 0 to end at 9
/// monotonically increasing the value the entire time.
fn dfs(array: &ArcArray2<usize>, start: Idx) -> usize {
    let value = array[start];
    if value == 9 {
        return 1;
    }
    let shape = array.shape();
    let shape = (shape[0], shape[1]);

    four_neighbors(start, shape)
        .filter(|&idx| array.get(idx).is_some())
        .filter(|&idx| array[idx] == value + 1)
        .map(|idx| dfs(array, idx))
        .sum()
}

impl Runner for Day {
    type Input<'input> = (Arc<[Idx]>, ArcArray2<usize>);

    fn day() -> usize {
        10
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let v = input
            .lines()
            .map(str::as_bytes)
            .flat_map(|line| line.iter().copied().map(|b| parse_uint([b])))
            .collect();
        let array = ArcArray2::from_shape_vec((height, width), v).unwrap();
        let starts = array
            .indexed_iter()
            .filter(|(_, &v)| v == 0)
            .map(|(idx, _)| idx)
            .collect();

        Ok((starts, array))
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let (starts, array) = input.clone();
        Ok(starts
            .iter()
            .map(|&idx| dfs_with_visited(&array, idx))
            .sum())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let (starts, array) = input.clone();
        Ok(starts.iter().map(|&idx| dfs(&array, idx)).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                89010123\n\
                78121874\n\
                87430965\n\
                96549874\n\
                45678903\n\
                32019012\n\
                01329801\n\
                10456732\
            ";
            part1 = 36;
            part2 = 81;
    }

    prod_case! {
        part1 = 607;
        part2 = 1384;
    }
}
