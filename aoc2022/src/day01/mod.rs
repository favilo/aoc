use color_eyre::Result;
use itertools::Itertools;

use crate::{
    utils::{parse_int, top::TopK},
    Runner,
};

pub struct Day;

impl Runner for Day {
    type Input<'input> = TopK<usize, 3>;

    fn day() -> usize {
        1
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        let nums = input
            .lines()
            .map(&str::trim)
            .batching(|it| {
                Some(
                    it.take_while(|l| !l.is_empty())
                        .map(str::as_bytes)
                        .map(parse_int)
                        .sum(),
                )
            })
            .take_while(|&s| s > 0)
            .collect();
        Ok(nums)
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(*input.into_iter().next().unwrap())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input.into_iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::prod_case;

    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "\
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(24000, Day::part1(&input)?);
        assert_eq!(45000, Day::part2(&input)?);
        Ok(())
    }

    prod_case! {
        part1 = 70720;
        part2 = 207148;
    }
}
