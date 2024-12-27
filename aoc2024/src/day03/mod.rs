use miette::Result;
use winnow::{
    ascii::dec_uint,
    combinator::{alt, repeat, repeat_till, rest, terminated},
    seq,
    stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial},
    token::any,
    PResult, Parser,
};

use crate::errors::ToMiette;
use aoc_utils::Runner;

pub struct Day;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Mul(usize, usize),
    Do,
    Dont,
}

impl Instruction {
    #[inline]
    #[must_use]
    pub fn total(&self) -> usize {
        match self {
            Instruction::Mul(a, b) => a * b,
            Instruction::Do => 0,
            Instruction::Dont => 0,
        }
    }

    fn parser<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr,
    {
        alt((Self::mul, Self::r#do, Self::dont)).parse_next(input)
    }

    fn mul<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr,
    {
        seq!(_: "mul(", dec_uint, _: ",", dec_uint, _: ")")
            .map(|(a, b)| Instruction::Mul(a, b))
            .parse_next(input)
    }

    fn r#do<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr,
    {
        "do()".value(Instruction::Do).parse_next(input)
    }

    fn dont<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr,
    {
        "don't()".value(Instruction::Dont).parse_next(input)
    }
}

fn all_instructions<S>(input: &mut S) -> PResult<<Day as Runner>::Input<'_>>
where
    for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
    <S as Stream>::Token: AsChar + Clone,
    <S as Stream>::Slice: AsBStr,
{
    terminated(
        repeat(
            0..,
            repeat_till(0.., any.value(()), Instruction::parser).map(|((), inst)| inst),
        ),
        rest,
    )
    .parse_next(input)
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AccumulatorState {
    Disabled = 0,
    #[default]
    Enabled = 1,
}

impl AccumulatorState {
    #[inline]
    fn should(&self) -> bool {
        *self == AccumulatorState::Enabled
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Processor {
    process: AccumulatorState,
}

impl Processor {
    #[inline]
    pub fn total(&mut self, inst: Instruction) -> usize {
        match inst {
            Instruction::Mul(_, _) => {
                if self.process.should() {
                    inst.total()
                } else {
                    0
                }
            }
            Instruction::Do => {
                self.process = AccumulatorState::Enabled;
                0
            }
            Instruction::Dont => {
                self.process = AccumulatorState::Disabled;
                0
            }
        }
    }
}

impl Runner for Day {
    type Input<'input> = Vec<Instruction>;

    fn day() -> usize {
        3
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        all_instructions.parse(input).to_miette()

        // For debugging with locations and spans:
        // use winnow::stream::Located;
        // all_instructions.parse(Located::new(input)).to_miette()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input.iter().map(Instruction::total).sum())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut processor = Processor::default();
        Ok(input
            .iter()
            .copied()
            .map(|inst| processor.total(inst))
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input1 = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
            part1 = 161;
            input2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
            part2 = 48;
    }

    prod_case! {
        part1 = 170778545;
        part2 = 82868252;
    }
}
