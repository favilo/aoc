use color_eyre::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, newline, space0, u64},
    combinator::{all_consuming, map, opt},
    error::VerboseError,
    multi::{many0, many1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use num::Integer;

use crate::{parsers::number, utils::top::TopK, Runner};

pub struct Day;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    N(u64),
    Old,
}
impl Operand {
    fn level(&self, worry: &u64) -> u64 {
        match self {
            Operand::N(n) => *n,
            Operand::Old => *worry,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Plus(Operand, Operand),
    Times(Operand, Operand),
}
impl Op {
    pub(crate) fn apply(&self, worry: &u64) -> u64 {
        match self {
            Op::Plus(o1, o2) => o1.level(worry) + o2.level(worry),
            Op::Times(o1, o2) => o1.level(worry) * o2.level(worry),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Monkey {
    id: usize,
    items: Vec<u64>,
    op: Op,
    test: u64,
    branches: [usize; 2],
    touched: usize,
}

fn monkey_id(input: &str) -> IResult<&str, usize, VerboseError<&str>> {
    delimited(tag("Monkey "), number, terminated(tag(":"), newline))(input)
}

fn starting_items(input: &str) -> IResult<&str, Vec<u64>, VerboseError<&str>> {
    delimited(
        terminated(space0, tag("Starting items: ")),
        many0(terminated(number::<_, u64, _>, opt(tag(", ")))),
        newline,
    )(input)
}

fn old(input: &str) -> IResult<&str, Operand, VerboseError<&str>> {
    map(tag("old"), |_| Operand::Old)(input)
}

fn operand(input: &str) -> IResult<&str, Operand, VerboseError<&str>> {
    terminated(alt((old, map(number, Operand::N))), space0)(input)
}

fn operation(input: &str) -> IResult<&str, Op, VerboseError<&str>> {
    delimited(
        tuple((space0, tag("Operation: new = "))),
        alt((
            map(tuple((operand, tag("* "), operand)), |(o1, _, o2)| {
                Op::Times(o1, o2)
            }),
            map(tuple((operand, tag("+ "), operand)), |(o1, _, o2)| {
                Op::Plus(o1, o2)
            }),
        )),
        newline,
    )(input)
}

fn test(input: &str) -> IResult<&str, u64, VerboseError<&str>> {
    delimited(
        tuple((space0, tag("Test: divisible by "))),
        u64,
        multispace0,
    )(input)
}

fn branches(input: &str) -> IResult<&str, [usize; 2], VerboseError<&str>> {
    let (input, (t, f)) = tuple((
        delimited(
            preceded(space0, tag("If true: throw to monkey ")),
            number,
            newline,
        ),
        delimited(
            preceded(space0, tag("If false: throw to monkey ")),
            number,
            newline,
        ),
    ))(input)?;
    Ok((input, [t, f]))
}

fn monkey(input: &str) -> IResult<&str, Monkey, VerboseError<&str>> {
    let (input, id) = monkey_id(input)?;
    let (input, items) = starting_items(input)?;
    let (input, op) = operation(input)?;
    let (input, test) = test(input)?;
    let (input, branches) = branches(input)?;
    let (input, _) = opt(newline)(input)?;
    Ok((
        input,
        Monkey {
            id,
            items,
            op,
            test,
            branches,
            touched: 0,
        },
    ))
}

impl Monkey {
    fn take_turn(&mut self) -> Vec<(usize, u64)> {
        // println!("Monkey {}", self.id);
        let moves = self
            .items
            .drain(..)
            .map(|worry| {
                let worry1 = self.op.apply(&worry);
                let worry2 = worry1 / 3;
                let test_idx = usize::from(worry2 % self.test != 0);
                let throw_to = self.branches[test_idx];
                (throw_to, worry2)
            })
            .collect_vec();
        self.touched += moves.len();
        moves
    }

    fn take_turn_no_worry_div(&mut self, modulus: u64) -> Vec<(usize, u64)> {
        // println!("Monkey {}", self.id);
        let moves = self
            .items
            .drain(..)
            .map(|worry| {
                let worry1 = self.op.apply(&worry);
                let worry2 = worry1 % modulus;
                let test_idx = usize::from(worry1 % self.test != 0);
                let throw_to = self.branches[test_idx];
                (throw_to, worry2)
            })
            .collect_vec();
        self.touched += moves.len();
        moves
    }

    fn accept_item(&mut self, item: u64) {
        self.items.push(item);
    }
}

fn do_round(monkeys: &mut Vec<Option<Monkey>>) {
    for monkey_id in 0..monkeys.len() {
        let mut this_monkey = monkeys[monkey_id].take().unwrap();
        this_monkey.take_turn().iter().for_each(|(m, w)| {
            let mut this_monkey = monkeys[*m].take().unwrap();
            this_monkey.accept_item(*w);
            monkeys[*m].replace(this_monkey);
        });
        monkeys[monkey_id].replace(this_monkey);
    }
}

fn do_round2(monkeys: &mut Vec<Option<Monkey>>, modulus: u64) {
    for monkey_id in 0..monkeys.len() {
        let mut this_monkey = monkeys[monkey_id].take().unwrap();
        this_monkey
            .take_turn_no_worry_div(modulus)
            .iter()
            .for_each(|(m, w)| {
                let mut this_monkey = monkeys[*m].take().unwrap();
                this_monkey.accept_item(*w);
                monkeys[*m].replace(this_monkey);
            });
        monkeys[monkey_id].replace(this_monkey);
    }
}

impl Runner for Day {
    type Input<'input> = Vec<Option<Monkey>>;

    fn day() -> usize {
        11
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        let (_, monkeys) = all_consuming(many1(map(monkey, Some)))(input).unwrap();
        Ok(monkeys)
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut monkeys = input.clone();
        (1..=20).for_each(|_| {
            do_round(&mut monkeys);
        });
        let top = TopK::<usize, 2>::from_iter(monkeys.iter().map(|m| m.as_ref().unwrap().touched));
        Ok(top.into_iter().product())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut monkeys = input.clone();
        let modulus = monkeys
            .iter()
            .flatten()
            .fold(1, |acc, m| acc.lcm(&(m.test)));
        (1..=10_000).for_each(|_| {
            do_round2(&mut monkeys, modulus);
        });
        let top = TopK::<usize, 2>::from_iter(monkeys.iter().map(|m| m.as_ref().unwrap().touched));
        Ok(top.into_iter().product())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
            part1 = 10605;
            part2 = 2713310158;
    }

    prod_case! {
        part1 = 57348;
        part2 = 14106266886;
    }

    #[test]
    fn monkey0() {
        assert_eq!(
            monkey(
                "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

"
            )
            .unwrap()
            .1,
            Monkey {
                id: 0,
                items: vec![79, 98],
                op: Op::Times(Operand::Old, Operand::N(19)),
                test: 23,
                branches: [2, 3],
                touched: 0,
            }
        );
    }
}
