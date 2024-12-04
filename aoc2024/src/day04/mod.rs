use itertools::Itertools;
use miette::Result;
use ndarray::Array2;

use crate::Runner;

pub struct Day;

type Coord = (usize, usize);
type Delta = (isize, isize);

const PART1_TARGET: [char; 4] = ['X', 'M', 'A', 'S'];

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
        .map(move |(dx, dy)| {
            (
                (
                    (
                        // Coordinate of middle
                        (idx.0 as isize + dx) as usize,
                        (idx.1 as isize + dy) as usize,
                    ),
                    (dx, dy),
                ),
                (0..N as isize)
                    .map(|i| {
                        (
                            (idx.0 as isize + dx * i) as usize,
                            (idx.1 as isize + dy * i) as usize,
                        )
                    })
                    .collect::<Vec<Coord>>()
                    .try_into()
                    .unwrap(),
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
        Ok(input
            .indexed_iter()
            .filter(|&(_, c)| c == &'X')
            .map(|(i, _)| index_iter::<4>(i, (input.nrows(), input.ncols())))
            .flat_map(|idxs| {
                idxs.map(|(_, idx)| {
                    idx.into_iter()
                        .map(|i| input[i])
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
            })
            .filter(|v: &[char; 4]| v == &PART1_TARGET)
            .count())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let crosses = input
            .indexed_iter()
            .filter(|&(_, c)| c == &'M')
            .map(|(i, _)| index_iter::<3>(i, (input.nrows(), input.ncols())))
            .flat_map(|idxs| {
                idxs.map(|(dir, idx)| {
                    (
                        dir,
                        idx.into_iter()
                            .map(|i| input[i])
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap(),
                    )
                })
            })
            .filter(|(_, v): &(_, [char; 3])| v == &['M', 'A', 'S'])
            .collect::<Vec<_>>();
        let crosses = crosses
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| a.0 .0 == b.0 .0)
            .filter(|(a, b)| perpendicular(a.0 .1, b.0 .1));
        Ok(crosses.count())
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
        part1 = 1681;
        part2 = 201684;
    }
}
