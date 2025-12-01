use miette::Result;

use aoc_utils::Runner;

pub struct Day;

impl Runner for Day {
    type Input<'input> = Vec<()>;

    #[rustfmt::skip]
    fn day() -> usize {
        {{ day }}
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
    use aoc_utils::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = indoc::indoc! {"
                <REPLACE ME>
            "};
            part1 = 0;
            part2 = 0;
    }

    prod_case! {
        part1 = 0;
        part2 = 0;
    }
}
