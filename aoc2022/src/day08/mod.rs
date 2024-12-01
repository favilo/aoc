use bit_set::BitSet;
use color_eyre::Result;
use ndarray::{
    parallel::prelude::{IntoParallelIterator, ParallelIterator},
    s, Array2, Zip,
};
use nom::{
    character::complete::multispace0, combinator::all_consuming, error::VerboseError, multi::many1,
    sequence::preceded, Finish,
};

use crate::{parsers::single_digit_line, Runner};

pub struct Day;

impl Runner for Day {
    type Input<'input> = Array2<usize>;

    fn day() -> usize {
        8
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        let v = all_consuming::<_, _, VerboseError<_>, _>(many1(preceded(
            multispace0,
            single_digit_line,
        )))(input.as_bytes())
        .finish()
        .unwrap()
        .1;
        let height = v.len();
        let width = v[0].len();
        let v = v.into_iter().flatten().collect::<Vec<_>>();
        let v = Array2::from_shape_vec((height, width), v)?;
        Ok(v)
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let (height, width) = input.dim();
        let mut set = BitSet::new();
        (0..height).into_iter().for_each(|j| {
            [s![j, ..], s![j, ..;-1]]
                .into_iter()
                .enumerate()
                .for_each(|(idx, s)| {
                    let mut max = -1;
                    input.slice(s).indexed_iter().for_each(|(i, &v)| {
                        let i = if idx == 1 { width - i - 1 } else { i };
                        if v as isize > max {
                            max = v as isize;
                            set.insert(i * width + j);
                        }
                    });
                });
        });
        (0..width).into_iter().for_each(|i| {
            [s![.., i], s![..;-1, i]]
                .into_iter()
                .enumerate()
                .for_each(|(idx, s)| {
                    let mut max = -1;
                    input
                        .slice(s)
                        .indexed_iter()
                        .for_each(|(j, &v): (usize, &usize)| {
                            let j = if idx == 1 { height - j - 1 } else { j };
                            if v as isize > max {
                                max = v as isize;
                                set.insert(i * width + j);
                            }
                        });
                });
        });

        Ok(set.len())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        Ok(Zip::indexed(input)
            .into_par_iter()
            .map(|(idx, e)| scenic_score(input, idx, *e))
            .max()
            .unwrap())
    }
}

fn scenic_score(array: &Array2<usize>, idx: (usize, usize), e: usize) -> usize {
    // TODO: I can see the shape of an algorithm that builds up the counts the same way
    // part1 did, but I'm not feeling motivated to implement it right now
    let (row, col) = idx;
    [
        s![..row;-1, col],
        s![row + 1.., col],
        s![row, ..col; -1],
        s![row, col + 1..; 1],
    ]
    .into_iter()
    .map(|idx| array.slice(idx))
    .map(|s| {
        let mut count = s.iter().take_while(|v| **v < e).count();
        // If we finish early, count the tree that we see last
        if count < s.len() {
            count += 1
        }
        count
    })
    .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                30373
                25512
                65332
                33549
                35390";
            part1 = 21;
            part2 = 8;
    }

    prod_case! {
        part1 = 1681;
        part2 = 201684;
    }
}
