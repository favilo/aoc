use std::collections::HashMap;

use color_eyre::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alpha1, line_ending, space0, u64},
    combinator::{all_consuming, map},
    error::VerboseError,
    multi::many1,
    sequence::tuple,
    IResult,
};
use nom_supreme::ParserExt;

use crate::Runner;

pub struct Day;

type SInput<'a> = &'a str;
type ParseResult<'a, T> = IResult<SInput<'a>, T, VerboseError<SInput<'a>>>;

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Plus,
    Minus,
    Times,
    Div,
}

#[derive(Debug, Clone)]
pub enum Expression {
    C(i64),
    Op(Op, String, String),
    Variable,
}

fn constant(input: SInput) -> ParseResult<Expression> {
    map(u64, |c| Expression::C(c as i64))(input)
}

fn op(input: SInput) -> ParseResult<Expression> {
    let (input, (name1, op, name2)) = tuple((
        alpha1.terminated(space0),
        take(1_usize),
        alpha1.preceded_by(space0),
    ))(input)?;

    let e = match op {
        "+" => Op::Plus,
        "-" => Op::Minus,
        "*" => Op::Times,
        "/" => Op::Div,
        _ => panic!("Bad operation"),
    };

    Ok((
        input,
        Expression::Op(e, name1.to_string(), name2.to_string()),
    ))
}

fn expression(input: SInput) -> ParseResult<Expression> {
    alt((constant, op))(input)
}

fn entry(input: SInput) -> ParseResult<(String, Expression)> {
    tuple((
        map(alpha1, str::to_string).terminated(tag(": ")),
        expression.terminated(line_ending),
    ))(input)
}

fn system(input: SInput) -> ParseResult<System> {
    map(many1(entry), |variables| System {
        variables: variables.into_iter().collect(),
    })(input)
}

#[derive(Debug, Clone)]
pub struct System {
    variables: HashMap<String, Expression>,
}

impl System {
    fn eval(&self, name: &str) -> i64 {
        match self.variables.get(name).unwrap() {
            Expression::C(c) => *c,
            Expression::Op(op, v1, v2) => match op {
                Op::Plus => self.eval(v1) + self.eval(v2),
                Op::Minus => self.eval(v1) - self.eval(v2),
                Op::Times => self.eval(v1) * self.eval(v2),
                Op::Div => self.eval(v1) / self.eval(v2),
            },
            // zero or panic!?
            Expression::Variable => 0,
        }
    }

    fn sides(&self) -> (&str, &str) {
        let Expression::Op(_, v1, v2) = self.variables.get("root").unwrap() else {panic!("Bad input")};
        (v1, v2)
    }

    fn evalable(&self, name: &str) -> bool {
        match self.variables.get(name).unwrap() {
            Expression::C(_) => true,
            Expression::Op(_, v1, v2) => self.evalable(v1) && self.evalable(v2),
            Expression::Variable => false,
        }
    }

    fn modify_for_part2(&mut self) {
        let humn = self.variables.get_mut("humn").unwrap();
        *humn = Expression::Variable;
    }

    fn solve_equation(&self, name: &str, c: i64) -> i64 {
        match self.variables.get(name).unwrap() {
            Expression::C(lit) => *lit,
            Expression::Op(op, lhs, rhs) => self.solve_op(*op, lhs, rhs, c),
            Expression::Variable => c,
        }
    }

    fn solve_op(&self, op: Op, lhs: &str, rhs: &str, c: i64) -> i64 {
        match op {
            Op::Plus => {
                let (var, new_c) = if self.evalable(lhs) {
                    (rhs, c - self.eval(lhs))
                } else {
                    (lhs, c - self.eval(rhs))
                };
                self.solve_equation(var, new_c)
            }
            Op::Minus => {
                let (var, new_c) = if self.evalable(lhs) {
                    (rhs, self.eval(lhs) - c)
                } else {
                    (lhs, c + self.eval(rhs))
                };
                self.solve_equation(var, new_c)
            }
            Op::Times => {
                let (var, new_c) = if self.evalable(lhs) {
                    (rhs, c / self.eval(lhs))
                } else {
                    (lhs, c / self.eval(rhs))
                };
                self.solve_equation(var, new_c)
            }
            Op::Div => {
                let (var, new_c) = if self.evalable(lhs) {
                    (rhs, self.eval(lhs) / c)
                } else {
                    (lhs, c * self.eval(rhs))
                };
                self.solve_equation(var, new_c)
            }
        }
    }
}

impl Runner<usize, i64> for Day {
    type Input<'input> = System;

    fn day() -> usize {
        21
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(all_consuming(system)(input).unwrap().1)
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input.eval("root") as usize)
    }

    fn part2(input: &Self::Input<'_>) -> Result<i64> {
        let mut input = input.clone();
        input.modify_for_part2();
        let (left, right) = input.sides();
        let (var, c) = if input.evalable(left) {
            (right, input.eval(left))
        } else {
            (left, input.eval(right))
        };
        Ok(input.solve_equation(var, c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                root: pppw + sjmn\n\
                dbpl: 5\n\
                cczh: sllz + lgvd\n\
                zczc: 2\n\
                ptdq: humn - dvpt\n\
                dvpt: 3\n\
                lfqf: 4\n\
                humn: 5\n\
                ljgn: 2\n\
                sjmn: drzm * dbpl\n\
                sllz: 4\n\
                pppw: cczh / lfqf\n\
                lgvd: ljgn * ptdq\n\
                drzm: hmdt - zczc\n\
                hmdt: 32\n";
            part1 = 152;
            part2 = 301;
    }

    prod_case! {
        part1 = 72664227897438;
        part2 = 3916491093817;
    }
}
