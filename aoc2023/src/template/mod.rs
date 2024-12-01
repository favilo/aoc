use color_eyre::Result;

use crate::Runner;

pub struct Day;

impl Runner for Day {
    type Input<'input> = Vec<()>;

    fn day() -> usize {
        0 // FIXME
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        todo!()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        todo!()
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
            part1 = 7;
            part2 = 19;
    }

    prod_case! {
        part1 = 1681;
        part2 = 201684;
    }
}
