use std::{collections::BTreeMap, convert::TryInto};

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, space0},
    multi::{many1, many_m_n},
    sequence::{delimited, terminated, tuple},
    IResult,
};

use crate::Runner;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum MaskBit {
    Zero,
    One,
    X,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Mask([MaskBit; 36]);

impl Default for Mask {
    fn default() -> Self {
        Self([MaskBit::X; 36])
    }
}

impl MaskBit {
    pub fn from_char(c: char) -> Self {
        match c {
            '0' => Self::Zero,
            '1' => Self::One,
            'X' => Self::X,
            _ => panic!("Wrong thing entered"),
        }
    }
}

impl Mask {
    pub fn val(&self, mut v: u64) -> u64 {
        (0..36).for_each(|i| match self.0[35 - i] {
            MaskBit::Zero => v = v & !(1 << i),
            MaskBit::One => v = v | (1 << i),
            MaskBit::X => (),
        });
        v
    }

    pub fn val2(&self, v: u64) -> Vec<u64> {
        let num = self.0.iter().filter(|&&b| b == MaskBit::X).count();
        let num = num;
        let mut copies = vec![v; 1 << num as usize];
        let mut seen = 0;
        (0..36).for_each(|i| match self.0[35 - i] {
            MaskBit::Zero => (),
            MaskBit::One => {
                copies.iter_mut().for_each(|v| *v = *v | (1 << i));
            }
            MaskBit::X => {
                copies.iter_mut().zip(pattern(1 << num, seen)).for_each(
                    |(v, bit): (&mut u64, bool)| {
                        if bit {
                            *v = *v | 1 << i;
                        } else {
                            *v = *v & !(1 << i);
                        }
                    },
                );
                seen += 1;
            }
        });
        copies
    }
}

fn pattern(num: usize, seen: usize) -> Vec<bool> {
    let mut bit = false;
    let mut count = 0;
    let mut v = Vec::new();
    for _ in 0..num {
        if count >= 1 << (seen) {
            bit = !bit;
            count = 0;
        }
        // println!("{} {} {}", bit, count, seen);
        v.push(bit);
        count += 1;
    }
    v
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Mask(Mask),
    Mem(u64, u64),
}

#[derive(Debug)]
pub struct Machine {
    mem: BTreeMap<u64, u64>,
    mask: Mask,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            mask: Mask::default(),
            mem: Default::default(),
        }
    }

    pub fn step(&mut self, op: &Operation) -> &Self {
        match op {
            Operation::Mask(mask) => self.mask = mask.clone(),
            Operation::Mem(addr, val) => {
                self.mem.insert(*addr, self.mask.val(*val));
            }
        }
        self
    }

    pub fn run(&mut self, program: &[Operation]) -> &Self {
        for op in program {
            self.step(op);
        }
        self
    }

    pub fn step2(&mut self, op: &Operation) -> &Self {
        match op {
            Operation::Mask(mask) => self.mask = mask.clone(),
            Operation::Mem(addr, val) => {
                self.mask.val2(*addr).into_iter().for_each(|addr| {
                    self.mem.insert(addr, *val);
                });
            }
        }
        self
    }

    pub fn run2(&mut self, program: &[Operation]) -> &Self {
        for op in program {
            self.step2(op);
        }
        self
    }
}

fn mask(input: &str) -> IResult<&str, Operation> {
    let (input, mask): (&str, Vec<char>) = delimited(
        tuple((tag("mask"), multispace0, tag("="), multispace0)),
        many_m_n(36, 36, alt((char('0'), char('1'), char('X')))),
        multispace0,
    )(input)?;

    Ok((
        input,
        Operation::Mask(Mask(
            mask.into_iter()
                .map(MaskBit::from_char)
                .collect::<Vec<MaskBit>>()
                .try_into()
                .unwrap(),
        )),
    ))
}

fn mem(input: &str) -> IResult<&str, Operation> {
    let (input, mem) = delimited(tag("mem["), digit1, tuple((tag("] ="), space0)))(input)?;
    let mem = mem.parse::<u64>().unwrap();
    let (input, b) = terminated(digit1, multispace0)(input)?;
    let b = b.parse::<u64>().unwrap();
    Ok((input, Operation::Mem(mem, b)))
}

fn parse_program(input: &str) -> IResult<&str, Vec<Operation>> {
    many1(terminated(alt((mem, mask)), multispace0))(input)
}

pub struct Day14;

impl Runner for Day14 {
    type Input = Vec<Operation>;
    type Output = u64;

    fn day() -> usize {
        14
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(parse_program(input).unwrap().1)
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        let mut machine = Machine::new();
        machine.run(input);
        Ok(machine.mem.values().sum())
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let mut machine = Machine::new();
        machine.run2(input);
        // println!("{:#?}", machine.mem);
        Ok(machine.mem.values().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
                     mem[8] = 11\n\
                     mem[7] = 101\n\
                     mem[8] = 0";

        let input = Day14::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(165, Day14::part1(&input)?);
        Ok(())
    }

    #[test]
    fn sample2() -> Result<()> {
        let input = "mask = 000000000000000000000000000000X1001X\n\
                     mem[42] = 100\n\
                     mask = 00000000000000000000000000000000X0XX\n\
                     mem[26] = 1";

        let input = Day14::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(208, Day14::part2(&input)?);
        Ok(())
    }
}
