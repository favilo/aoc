use miette::Result;
use winnow::{
    ascii::{dec_uint, line_ending, space0},
    combinator::{repeat, separated, separated_pair},
    seq, PResult, Parser,
};

use crate::{errors::ToMiette, Runner};

pub struct Day;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Equation {
    total: usize,
    operands: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Continuation {
    Done,
    Error,
}

impl Equation {
    fn possible(&self) -> bool {
        // Backtracking algorithm to find if add or multiply is needed
        self.solve(
            &self.operands,
            &mut vec![],
            &[Operator::Add, Operator::Multiply],
        ) == Continuation::Done
    }

    fn possible_with_concat(&self) -> bool {
        // Backtracking algorithm to find if add or multiply is needed
        self.solve(
            &self.operands,
            &mut vec![],
            &[Operator::Add, Operator::Multiply, Operator::Concat],
        ) == Continuation::Done
    }

    /// Recursive function that figures out the correct operators to use
    fn solve(
        &self,
        operands: &[usize],
        operators: &mut Vec<Operator>,
        choices: &[Operator],
    ) -> Continuation {
        if operands.len() == 1 {
            return if self.apply(&self.operands, operators) == self.total {
                Continuation::Done
            } else {
                Continuation::Error
            };
        }

        let operands_count = operands.len();
        let total_operands = self.operands.len();
        let left_operands = total_operands - operands_count;
        // dbg!(operands_count, total_operands, left_operands);
        // dbg!(&self.operands, operands, &operators);

        if !operators.is_empty()
            && self.apply(&self.operands[..left_operands], operators) >= self.total
        {
            return Continuation::Error;
        }

        for operator in choices {
            operators.push(*operator);
            match self.solve(&operands[1..], operators, choices) {
                Continuation::Done => return Continuation::Done,
                Continuation::Error => {}
            }
            operators.pop();
        }
        Continuation::Error
    }

    fn apply(&self, operands: &[usize], operators: &[Operator]) -> usize {
        let mut total = operands[0];
        for (&a, operator) in operands[1..].iter().zip(operators.iter().rev()) {
            match operator {
                Operator::Add => total += a,
                Operator::Multiply => total *= a,
                Operator::Concat => total = format!("{total}{a}").parse().unwrap(),
            }
        }
        total
    }
}

fn equation(input: &mut &str) -> PResult<Equation> {
    seq!(
        separated_pair(
            dec_uint,
            (":", space0),
            separated(1.., dec_uint::<_, usize, _>, " "),
        ),
        _: line_ending
    )
    .map(|t| Equation {
        total: t.0 .0,
        operands: t.0 .1,
    })
    .parse_next(input)
}

impl Runner for Day {
    type Input<'input> = Vec<Equation>;

    fn day() -> usize {
        7
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        repeat(1.., equation).parse(input).to_miette()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .iter()
            .filter(|&eq| eq.possible())
            .map(|eq| eq.total)
            .sum())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .iter()
            .filter(|&eq| eq.possible_with_concat())
            .map(|eq| eq.total)
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                190: 10 19\n\
                3267: 81 40 27\n\
                83: 17 5\n\
                156: 15 6\n\
                7290: 6 8 6 15\n\
                161011: 16 10 13\n\
                192: 17 8 14\n\
                21037: 9 7 18 13\n\
                292: 11 6 16 20\n\
            ";
            part1 = 3749;
            part2 = 11387;
    }

    prod_case! {
        part1 = 3598800864292;
        part2 = 340362529351427;
    }
}
