use miette::Result;

use aoc_utils::{errors::ToMiette, Runner};
use winnow::Parser;

use crate::intcode::Program;

pub struct Day;

impl Runner for Day {
    type Input<'input> = Program;

    #[rustfmt::skip]
    fn day() -> usize {
        5
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Program::parser.parse(input).to_miette()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut program = input.clone();
        let v = program.run(&mut &[1][..])?;
        v.last()
            .map(|v| *v as usize)
            .ok_or(miette::miette!("No outputs"))
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut program = input.clone();
        let v = program.run(&mut &[5][..])?;
        v.last()
            .map(|v| *v as usize)
            .ok_or(miette::miette!("No outputs"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = indoc::indoc! {"
                3,0,4,0,99
            "};
            part1 = 1;
            part2 = 5;
    }

    sample_case! {
        jump_test =>
            input = indoc::indoc! {"
                3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9
            "};
            part1 = 1;
            part2 = 1;
    }

    sample_case! {
        jump_test_immediate =>
            input = indoc::indoc! {"
                3,3,1105,-1,9,1101,0,0,12,4,12,99,1
            "};
            part1 = 1;
            part2 = 1;
    }

    sample_case! {
        larger_example =>
            input = indoc::indoc! {"
                3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
            "};
            part1 = 999;
            part2 = 999;
    }

    prod_case! {
        part1 = 5074395;
        part2 = 8346937;
    }
}
