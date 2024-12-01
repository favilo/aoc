use std::collections::{HashSet, VecDeque};

use color_eyre::Result;
use itertools::Itertools;

use crate::{
    utils::{parse_int, six_neighbors},
    Runner,
};

pub struct Day;

impl Runner for Day {
    type Input<'input> = HashSet<[isize; 3]>;

    fn day() -> usize {
        18
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        let droplets: HashSet<[isize; 3]> = input
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|n| parse_int(n.as_bytes()) as isize)
                    .collect_vec()
                    .try_into()
                    .unwrap()
            })
            .collect();

        Ok(droplets)
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .into_iter()
            .map(|cell| six_neighbors(*cell).filter(|n| !input.contains(n)).count())
            .sum())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let outside = outside(&input);
        // dbg!(input.len(), outside.len(), 21 * 21 * 21);
        // visualize(input, outside);
        Ok(input
            .into_iter()
            .flat_map(|cell| six_neighbors(*cell))
            .filter(|n| outside.contains(n))
            .count())
    }
}

fn outside(droplets: &HashSet<[isize; 3]>) -> HashSet<[isize; 3]> {
    let (mins, maxes) = bounding_boxes(droplets);
    let mut outside_points = HashSet::new();
    let mut queue = VecDeque::new();
    let start = mins;
    queue.push_back(start);
    let mut visited = HashSet::new();
    visited.insert(start);
    outside_points.insert(start);
    while let Some(coord) = queue.pop_back() {
        six_neighbors(coord)
            .filter(|c| (0..3).all(|i| c[i] >= mins[i] - 1 && c[i] <= maxes[i] + 1))
            .filter(|c| !droplets.contains(c))
            .for_each(|c| {
                if visited.insert(c) {
                    queue.push_back(c);
                    outside_points.insert(c);
                }
            })
    }
    outside_points
}

fn bounding_boxes(droplets: &HashSet<[isize; 3]>) -> ([isize; 3], [isize; 3]) {
    let bounding_box: [(isize, isize); 3] = [0, 1, 2]
        .iter()
        .map(|dim| -> (isize, isize) {
            (
                droplets.iter().map(|d| d[*dim]).min().unwrap(),
                droplets.iter().map(|d| d[*dim]).max().unwrap(),
            )
        })
        .collect_vec()
        .try_into()
        .unwrap();
    let [(x0, x1), (y0, y1), (z0, z1)] = bounding_box;
    let mins = [x0, y0, z0];
    let maxes = [x1, y1, z1];
    (mins, maxes)
}

#[allow(dead_code)]
fn visualize(droplets: &HashSet<[isize; 3]>, outside: &HashSet<[isize; 3]>) {
    println!("=========BEGIN VISUALIZATION========");
    (0..=20).for_each(|z: isize| {
        println!("\nz = {z}");
        (0..=20).for_each(|y: isize| {
            (0..=20).for_each(|x: isize| {
                if droplets.contains(&[x, y, z]) {
                    print!("X");
                } else if !outside.contains(&[x, y, z]) {
                    print!("O");
                } else {
                    print!(".");
                }
            });
            println!();
        });
    });
    println!("========END VISUALIZATION========");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                2,2,2\n\
                1,2,2\n\
                3,2,2\n\
                2,1,2\n\
                2,3,2\n\
                2,2,1\n\
                2,2,3\n\
                2,2,4\n\
                2,2,6\n\
                1,2,5\n\
                3,2,5\n\
                2,1,5\n\
                2,3,5\n";
            part1 = 64;
            part2 = 58;
    }

    prod_case! {
        part1 = 3432;
        part2 = 2042;
    }
}
