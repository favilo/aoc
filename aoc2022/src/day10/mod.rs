use std::{fmt::Write, iter::once};

use color_eyre::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::map,
    error::VerboseError,
    sequence::{delimited, preceded},
    IResult,
};

use crate::{parsers::signed_number, Runner};

pub struct Day;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Noop,
    AddX(isize),
}

#[derive(Debug, Clone, Copy)]
pub enum ModifiedInstruction {
    Noop,
    Pause,
    AddX(isize),
}

impl ModifiedInstruction {
    pub fn transform(inst: &[Instruction]) -> Vec<Self> {
        inst.iter()
            .flat_map(|&i| match i {
                Instruction::Noop => vec![Self::Noop],
                Instruction::AddX(a) => vec![Self::Pause, Self::AddX(a)],
            })
            .collect_vec()
    }
}

fn addx(input: &[u8]) -> IResult<&[u8], Instruction, VerboseError<&[u8]>> {
    map(preceded(tag("addx "), signed_number), Instruction::AddX)(input)
}

fn noop(input: &[u8]) -> IResult<&[u8], Instruction, VerboseError<&[u8]>> {
    map(tag("noop"), |_| Instruction::Noop)(input)
}

fn instruction(input: &[u8]) -> IResult<&[u8], Instruction, VerboseError<&[u8]>> {
    delimited(multispace0, alt((addx, noop)), multispace0)(input)
}

#[derive(Debug, Clone)]
pub struct Program<'input> {
    instructions: &'input [Instruction],
}

impl<'input> Program<'input> {
    pub fn new(instructions: &'input [Instruction]) -> Program<'input> {
        Self { instructions }
    }

    pub fn cycles_iter(&mut self) -> impl Iterator<Item = isize> + '_ {
        let mut x = 1;
        once(x).chain(self.instructions.iter().flat_map(move |i| match i {
            Instruction::Noop => vec![x],
            Instruction::AddX(addend) => {
                let a = vec![x, x + addend];
                x += addend;
                a
            }
        }))
    }

    pub fn inst_iter(&mut self) -> impl Iterator<Item = Instruction> + '_ {
        once(Instruction::Noop).chain(self.instructions.iter().flat_map(move |&i| match i {
            Instruction::Noop => {
                vec![i]
            }
            Instruction::AddX(_) => {
                vec![i, i]
            }
        }))
    }
}

impl Runner<isize, ()> for Day {
    type Input<'input> = Vec<Instruction>;

    fn day() -> usize {
        10
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(input
            .lines()
            .map(|l| instruction(l.as_ref()).unwrap().1)
            .collect())
    }

    fn part1(input: &Self::Input<'_>) -> Result<isize> {
        let mut p = Program::new(input);
        let cycles = p.cycles_iter().collect_vec();
        let sum = [20, 60, 100, 140, 180, 220]
            .into_iter()
            .map(|cycle| cycles[cycle - 1] * cycle as isize)
            .sum();
        Ok(sum)
    }

    fn part2(input: &Self::Input<'_>) -> Result<()> {
        let input = ModifiedInstruction::transform(input);
        let mut cycle = 0;
        let mut sprite = 1;
        let mut screen = String::new();
        assert_eq!(input.len(), 240);
        for inst in input {
            // print!("Sprite position: ");
            // for i in 0..40 {
            //     print!(
            //         "{}",
            //         if i <= sprite + 1 && i >= sprite - 1 {
            //             "#"
            //         } else {
            //             "."
            //         }
            //     );
            // }
            // println!("\n");

            // 1. Cycle starts
            cycle += 1;
            // println!("Start cycle {cycle:2}: begin executing {inst:?}");
            let x_pos = (cycle % 40) - 1;

            // 2. Pixel is drawn
            if x_pos <= (sprite + 1) && x_pos >= (sprite - 1) {
                write!(screen, "#")?;
            } else {
                write!(screen, ".")?;
            }
            if cycle != 0 && cycle % 40 == 0 {
                writeln!(screen)?;
            }

            // 3. Instruction Handled
            match inst {
                ModifiedInstruction::Noop | ModifiedInstruction::Pause => {}
                ModifiedInstruction::AddX(a) => sprite += a,
            }
        }

        // println!("{}", screen);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
    sample1 =>
        input = "addx 15
                    addx -11
                    addx 6
                    addx -3
                    addx 5
                    addx -1
                    addx -8
                    addx 13
                    addx 4
                    noop
                    addx -1
                    addx 5
                    addx -1
                    addx 5
                    addx -1
                    addx 5
                    addx -1
                    addx 5
                    addx -1
                    addx -35
                    addx 1
                    addx 24
                    addx -19
                    addx 1
                    addx 16
                    addx -11
                    noop
                    noop
                    addx 21
                    addx -15
                    noop
                    noop
                    addx -3
                    addx 9
                    addx 1
                    addx -3
                    addx 8
                    addx 1
                    addx 5
                    noop
                    noop
                    noop
                    noop
                    noop
                    addx -36
                    noop
                    addx 1
                    addx 7
                    noop
                    noop
                    noop
                    addx 2
                    addx 6
                    noop
                    noop
                    noop
                    noop
                    noop
                    addx 1
                    noop
                    noop
                    addx 7
                    addx 1
                    noop
                    addx -13
                    addx 13
                    addx 7
                    noop
                    addx 1
                    addx -33
                    noop
                    noop
                    noop
                    addx 2
                    noop
                    noop
                    noop
                    addx 8
                    noop
                    addx -1
                    addx 2
                    addx 1
                    noop
                    addx 17
                    addx -9
                    addx 1
                    addx 1
                    addx -3
                    addx 11
                    noop
                    noop
                    addx 1
                    noop
                    addx 1
                    noop
                    noop
                    addx -13
                    addx -19
                    addx 1
                    addx 3
                    addx 26
                    addx -30
                    addx 12
                    addx -1
                    addx 3
                    addx 1
                    noop
                    noop
                    noop
                    addx -9
                    addx 18
                    addx 1
                    addx 2
                    noop
                    noop
                    addx 9
                    noop
                    noop
                    noop
                    addx -1
                    addx 2
                    addx -37
                    addx 1
                    addx 3
                    noop
                    addx 15
                    addx -21
                    addx 22
                    addx -6
                    addx 1
                    noop
                    addx 2
                    addx 1
                    noop
                    addx -10
                    noop
                    noop
                    addx 20
                    addx 1
                    addx 2
                    addx 2
                    addx -6
                    addx -11
                    noop
                    noop
                    noop";
        part1 = 13140;
        part2 = ();
    }

    prod_case! {
        part1 = 13060;
        part2 = ();
    }
}
