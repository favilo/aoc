use itertools::iterate;
use miette::{IntoDiagnostic, Result};

use aoc_utils::Runner;

pub struct Day;

fn mass_fuel(mass: &isize) -> isize {
    (mass / 3) - 2
}

fn total_fuel(fuel: &isize) -> isize {
    iterate(mass_fuel(fuel), mass_fuel)
        .take_while(|&fuel| fuel > 0)
        .sum::<isize>()
}

impl Runner for Day {
    type Input<'input> = Vec<isize>;

    #[rustfmt::skip]
    fn day() -> usize {
        1
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        input
            .lines()
            .map(|line| line.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()
            .into_diagnostic()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input.iter().map(mass_fuel).sum::<isize>() as usize)
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let fuel = input.iter().map(total_fuel).sum::<isize>();
        Ok(fuel as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = indoc::indoc! {"
                100756
            "};
            part1 = 33583;
            part2 = 50346;
    }

    prod_case! {
        part1 = 3318195;
        part2 = 4974428;
    }
}
