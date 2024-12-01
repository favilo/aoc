use color_eyre::Result;
use itertools::Itertools;

use crate::Runner;

pub struct Day;

fn decrypt<const N: usize>(numbers: &[i64], indices: &mut Vec<i64>, key: i64) {
    for _ in 0..N {
        for i in 0..(numbers.len() as i64) {
            let index = indices.iter().position(|&p| p == i).unwrap();
            indices.remove(index);
            let new_index = (index as i64 + numbers[i as usize] * key)
                .rem_euclid(indices.len() as i64) as usize;
            indices.insert(new_index, i);
        }
    }
}

impl Runner for Day {
    type Input<'input> = Vec<i64>;

    fn day() -> usize {
        20
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(input
            .lines()
            .map(str::trim)
            .map(|s| s.parse::<i64>().unwrap())
            .collect())
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut indices = (0..(input.len() as i64)).collect_vec();
        decrypt::<1>(input, &mut indices, 1);
        let zero_p = input.iter().position(|&v| v == 0).unwrap();
        let zero = indices.iter().position(|&p| p == zero_p as i64).unwrap();
        Ok([1000, 2000, 3000]
            .iter()
            .map(|i| input[indices[(zero + i) as usize % indices.len()] as usize] as i64)
            .sum::<i64>() as usize)
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut indices = (0..(input.len() as i64)).collect_vec();
        let key = 811589153;
        decrypt::<10>(input, &mut indices, key);
        let zero_p = input.iter().position(|&v| v == 0).unwrap();
        let zero = indices.iter().position(|&p| p == zero_p as i64).unwrap();
        Ok([1000, 2000, 3000]
            .iter()
            .map(|i| input[indices[(zero + i).rem_euclid(indices.len())] as usize] * key)
            .sum::<i64>() as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                1\n\
                2\n\
                -3\n\
                3\n\
                -2\n\
                0\n\
                4\n";
            part1 = 3;
            part2 = 1623178306;
    }

    prod_case! {
        part1 = 14888;
        part2 = 3760092545849;
    }
}
