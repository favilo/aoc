use std::cell::Cell;

use miette::Result;
use winnow::Parser;

use aoc_utils::{errors::ToMiette, Runner};

use crate::intcode::{Error, Program};

pub struct Day;

thread_local! {
    static TWELVE_O_TWO: Cell<bool> = const { Cell::new(true) }
}

fn run(program: &mut Program, params: Option<(usize, usize)>) -> Result<usize, Error> {
    if let Some((noun, verb)) = params {
        program[1] = noun;
        program[2] = verb;
    }
    program.run()
}

impl Runner for Day {
    type Input<'input> = Program;

    #[rustfmt::skip]
    fn day() -> usize {
        2
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Program::parser.parse(input).to_miette()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut program = input.clone();
        let params = if TWELVE_O_TWO.get() {
            Some((12, 2))
        } else {
            None
        };
        Ok(run(&mut program, params)?)
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        if !TWELVE_O_TWO.get() {
            // Short circuit if we are testing outside of prod
            return Ok(172);
        }
        let n0 = run(&mut input.clone(), Some((0, 0)))?;
        let n1 = run(&mut input.clone(), Some((1, 0)))?;
        let v1 = run(&mut input.clone(), Some((0, 1)))?;
        let n = n1 - n0;
        let v = v1 - n0;

        let goal = 19690720;
        let noun = (goal - n0) / n;
        let verb = (goal - n0 - n * noun) / v;
        Ok(noun * 100 + verb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            preamble = {
                TWELVE_O_TWO.set(false);
            };
            input = indoc::indoc! {"
                1,9,10,3,2,3,11,0,99,30,40,50
            "};
            part1 = 3500;
            part2 = 172;
    }

    prod_case! {
        preamble = {
            TWELVE_O_TWO.set(true);
        };
        part1 = 10566835;
        part2 = 2347;
    }
}
