use anyhow::Result;
use itertools::Itertools;

use crate::Runner;

pub struct Day;

impl Runner<i32, i32> for Day {
    type Input = Vec<i32>;

    fn day() -> usize {
        1
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        let nums = input
            .lines()
            .map(&str::trim)
            .map(|l| i32::from_str_radix(l, 10).unwrap())
            .collect();
        Ok(nums)
    }

    fn part1(input: &Self::Input) -> Result<i32> {
        let total = input.iter().fold((0, None), |acum, i| {
            if acum.1.is_none() {
                (0, Some(i))
            } else {
                if i > acum.1.unwrap() {
                    (acum.0 + 1, Some(i))
                } else {
                    (acum.0, Some(i))
                }
            }
        });
        Ok(total.0)
    }

    fn part2(input: &Self::Input) -> Result<i32> {
        let total = input
            .iter()
            .tuple_windows()
            .map(|(&a, &b, &c)| a + b + c)
            .fold((0, None), |acum, i| {
                if acum.1.is_none() {
                    (0, Some(i))
                } else {
                    if i > acum.1.unwrap() {
                        (acum.0 + 1, Some(i))
                    } else {
                        (acum.0, Some(i))
                    }
                }
            });
        Ok(total.0)
    }
}
