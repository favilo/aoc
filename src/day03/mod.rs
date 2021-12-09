use anyhow::Result;

use crate::Runner;

#[inline]
fn most_common_bit(input: &<Day as Runner>::Input, bit: u8) -> u8 {
    let len = input.len();
    let ones: usize = input.iter().map(|num| (num >> bit) & 1).sum();
    if ones >= (len - ones) {
        1
    } else {
        0
    }
}

#[inline]
fn filter_whats_left(left: &mut <Day as Runner>::Input, digit: u8, bit: usize) {
    *left = left
        .iter()
        .copied()
        .filter(|num| ((*num >> digit) & 1) == bit)
        .collect();
}

pub struct Day;
impl Runner for Day {
    type Input = Vec<usize>;
    type Output = usize;

    fn day() -> usize {
        03
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .lines()
            .map(|line| usize::from_str_radix(line, 2))
            .map(Result::unwrap)
            .collect())
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        let gamma = (0..12)
            .rev()
            .map(|b| most_common_bit(input, b))
            .fold(0usize, |int, digit| (int << 1) + digit as usize);

        let epsilon = (!gamma as usize) & 0b1111_1111_1111;
        let answer = gamma * epsilon;
        Ok(answer)
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let mut left = input.to_owned();
        let bits = 12;
        let oxygen = (0..bits)
            .rev()
            .find_map(|digit| {
                let bit = most_common_bit(&left, digit) as usize;
                filter_whats_left(&mut left, digit, bit);
                if left.len() == 1 {
                    return Some(left[0]);
                }
                None
            })
            .unwrap();
        left = input.to_owned();
        let co2 = (0..bits)
            .rev()
            .find_map(|digit| {
                let bit = 1 - most_common_bit(&left, digit) as usize;
                filter_whats_left(&mut left, digit, bit);
                if left.len() == 1 {
                    return Some(left[0]);
                }
                None
            })
            .unwrap();
        let answer = oxygen * co2;
        Ok(answer)
    }
}
