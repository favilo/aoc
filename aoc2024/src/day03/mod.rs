use miette::Result;
use winnow::{
    ascii::dec_uint,
    combinator::{alt, eof, fail, opt, repeat, rest, trace},
    error::ErrMode,
    seq,
    stream::Stream,
    token::{any, take_till},
    Located, PResult, Parser,
};

use crate::errors::ToMiette;
use crate::Runner;

pub struct Day;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Mul(usize, usize),
    Do,
    Dont,
}

fn mul(input: &mut Located<&str>) -> PResult<Instruction> {
    seq!(_: "mul(", dec_uint, _: ",", dec_uint, _: ")")
        .map(|(a, b)| Instruction::Mul(a, b))
        .parse_next(input)
}

fn r#do(input: &mut Located<&str>) -> PResult<Instruction> {
    seq!(_: "do()").map(|_| Instruction::Do).parse_next(input)
}

fn dont(input: &mut Located<&str>) -> PResult<Instruction> {
    seq!(_: "don't()")
        .map(|_| Instruction::Dont)
        .parse_next(input)
}

fn instruction(input: &mut Located<&str>) -> PResult<Instruction> {
    alt((mul, r#do, dont)).parse_next(input)
}

fn not_instruction(input: &mut Located<&str>) -> PResult<()> {
    while !input.is_empty() {
        let instruction_letters = ('m'..='m', 'd'..='d');
        let v: PResult<&str> = take_till(0.., instruction_letters).parse_next(input);
        match v {
            Err(ErrMode::Backtrack(_)) => {
                rest.parse_next(input)?;
                return Ok(());
            }
            Err(e) => return Err(e),
            Ok(_) => {}
        }

        let checkpoint = input.checkpoint();
        let v = instruction.with_taken().parse_next(input);
        match v {
            Err(_) => {}
            Ok(_) => {
                input.reset(&checkpoint);
                return Ok(());
            }
        }
        let v = any.parse_next(input);
        match v {
            Err(ErrMode::Backtrack(_)) => {
                eof.parse_next(input)?;
                return Ok(());
            }
            Err(e) => return Err(e),
            Ok(_) => {}
        }
    }
    fail.parse_next(input)
}

fn all_instructions(input: &mut Located<&str>) -> PResult<Vec<Instruction>> {
    repeat(
        0..,
        seq!(_: trace("before instruction()", not_instruction), opt(instruction)).map(|t| t.0),
    )
    .map(|v: Vec<Option<_>>| v.into_iter().flatten().collect())
    .parse_next(input)
}

impl Runner for Day {
    type Input<'input> = Vec<Instruction>;

    fn day() -> usize {
        3
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        all_instructions.parse(Located::new(input)).to_miette()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .iter()
            .filter(|i| matches!(i, Instruction::Mul(_, _)))
            .map(|i| {
                let Instruction::Mul(a, b) = i else {
                    unreachable!()
                };
                a * b
            })
            .sum())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut do_it = true;
        let mut sum = 0;
        for inst in input {
            match inst {
                Instruction::Mul(a, b) => {
                    if do_it {
                        sum += a * b;
                    }
                }
                Instruction::Do => {
                    do_it = true;
                }
                Instruction::Dont => {
                    do_it = false;
                }
            }
        }
        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
            part1 = 161;
            part2 = 161;
    }

    sample_case! {
        sample2 =>
            input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
            part1 = 161;
            part2 = 48;
    }

    prod_case! {
        part1 = 170778545;
        part2 = 82868252;
    }
}
