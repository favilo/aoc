use std::collections::HashMap;

use itertools::Itertools;
use miette::Result;
use ndarray::Array2;

use crate::Runner;

pub struct Day;

type Coord = (usize, usize);
type Delta = (isize, isize);

const PART1_TARGET: [char; 4] = ['X', 'M', 'A', 'S'];
const PART2_TARGET: [char; 3] = ['M', 'A', 'S'];

fn index_iter<const N: usize>(
    idx: Coord,
    dim: Coord,
) -> impl Iterator<Item = ((Coord, Delta), [Coord; N])> {
    [-1_isize, 0, 1]
        .into_iter()
        .cartesian_product([-1, 0, 1])
        .filter(|d| d != &(0, 0))
        .filter(move |(dx, dy)| {
            let xmax = dx * (N as isize - 1) + idx.0 as isize;
            let ymax = dy * (N as isize - 1) + idx.1 as isize;
            xmax < dim.0 as isize && xmax >= 0 && ymax < dim.1 as isize && ymax >= 0
        })
        .map(move |delta @ (dx, dy)| {
            let mut coords = [Default::default(); N];
            let mut map = (0..N as isize).map(|i| {
                (
                    (idx.0 as isize + dx * i) as usize,
                    (idx.1 as isize + dy * i) as usize,
                )
            });
            coords.fill_with(|| map.next().expect("array is the right size"));
            let middle_coords = (
                // Coordinate of middle, for part2
                (idx.0 as isize + dx) as usize,
                (idx.1 as isize + dy) as usize,
            );
            (
                (
                    middle_coords,
                    // Direction, for part2
                    delta,
                ),
                // Coordinates of the four surrounding cells, for parts 1 and 2
                coords,
            )
        })
}

fn perpendicular(a: Delta, b: Delta) -> bool {
    match a {
        (0, 1) | (0, -1) => [(0, 1), (0, -1)].contains(&b),
        (1, 0) | (-1, 0) => [(1, 0), (-1, 0)].contains(&b),
        (1, 1) | (-1, -1) => [(1, -1), (-1, 1)].contains(&b),
        (1, -1) | (-1, 1) => [(1, 1), (-1, -1)].contains(&b),
        _ => panic!("invalid delta: {a:?} {b:?}"),
    }
}

fn slice_array_from_indexes<T, const N: usize>(
    input: &Array2<T>,
    idx: [(usize, usize); N],
) -> [T; N]
where
    T: Default + Copy,
{
    // I want to use std::array::fill_with, but it's was measurably slower in part1
    let mut output = [T::default(); N];
    let mut iter = idx.into_iter().map(|i| input[i]);
    output.fill_with(|| iter.next().expect("array is the right size"));
    output
}

impl Runner for Day {
    type Input<'input> = Array2<char>;

    fn day() -> usize {
        4
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let v = input.lines().flat_map(|line| line.chars()).collect();
        Ok(Array2::from_shape_vec((height, width), v).unwrap())
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let count = input
            .indexed_iter()
            .filter(|&(_, c)| c == &'X')
            .map(|(i, _)| index_iter::<4>(i, (input.nrows(), input.ncols())))
            .flat_map(|idxs| idxs.map(|(_, idx)| slice_array_from_indexes(input, idx)))
            .filter(|v: &[char; 4]| v == &PART1_TARGET)
            .count();
        Ok(count)
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut map = HashMap::new();
        input
            .indexed_iter()
            .filter(|&(_, c)| c == &'M')
            .map(|(i, _)| index_iter::<3>(i, (input.nrows(), input.ncols())))
            .flat_map(|idxs| idxs.map(|(dir, idx)| (dir, slice_array_from_indexes(input, idx))))
            .filter(|(_, v): &(_, [char; 3])| v == &PART2_TARGET)
            .map(|((coord, dir), _)| (coord, dir))
            .for_each(|(coord, dir)| {
                map.entry(coord).or_insert_with(Vec::new).push(dir);
            });

        Ok(map
            .iter()
            .filter(|&(_, v)| v.len() > 1)
            .flat_map(|(_, v)| {
                v.iter()
                    .tuple_combinations()
                    .filter(|&(a, b)| perpendicular(*a, *b))
            })
            .count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                MMMSXXMASM\n\
                MSAMXMSMSA\n\
                AMXSXMAAMM\n\
                MSAMASMSMX\n\
                XMASAMXAMM\n\
                XXAMMXXAMA\n\
                SMSMSASXSS\n\
                SAXAMASAAA\n\
                MAMMMXMMMM\n\
                MXMXAXMASX\n\
            ";
            part1 = 18;
            part2 = 9;
    }

    prod_case! {
        part1 = 2406;
        part2 = 1807;
    }
}
