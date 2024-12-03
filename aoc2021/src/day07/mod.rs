use anyhow::Result;

use crate::{
    utils::{mean, median, parse_int},
    Runner,
};

pub struct Day;

impl Runner for Day {
    type Input = Vec<usize>;

    fn day() -> usize {
        7
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        // Only good for valid data
        Ok(input
            .trim()
            .as_bytes()
            .split(|&c| ',' as u8 == c)
            // .map(|s| s.parse().unwrap())
            .map(parse_int)
            .collect())
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        let idx = median(input) - 1;
        Ok(input
            .iter()
            .copied()
            .map(|v| (v as isize - idx as isize).abs() as usize)
            .sum())
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        let floor = mean(input).floor() as usize;
        let ceil = mean(input).ceil() as usize;
        Ok([floor, ceil]
            .iter()
            .copied()
            .map(|idx| {
                input
                    .iter()
                    .copied()
                    .map(|v| {
                        let n = (v as isize - idx as isize).abs() as usize;
                        n * (n + 1) >> 1
                    })
                    .sum()
            })
            .min()
            .unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "16,1,2,0,4,2,7,1,2,14";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(37, Day::part1(&input)?);
        assert_eq!(168, Day::part2(&input)?);
        Ok(())
    }
}
