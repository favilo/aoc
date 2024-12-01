use color_eyre::Result;
use heapless::String;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, multispace1, newline},
    combinator::{all_consuming, map, opt},
    error::VerboseError,
    multi::many1,
    sequence::{delimited, preceded, terminated, tuple},
    Finish, IResult,
};

use crate::{parsers::number, Runner};

pub struct Day;

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    num: usize,
    from: usize,
    to: usize,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Crate(char);

#[derive(Debug, Clone, Default)]
pub struct Stacks {
    stacks: Vec<Vec<Crate>>,
    instructions: Vec<Instruction>,
}

fn container(input: &[u8]) -> IResult<&[u8], Crate, VerboseError<&[u8]>> {
    map(delimited(tag("["), anychar, tag("]")), Crate)(input)
}

fn hole(input: &[u8]) -> IResult<&[u8], (), VerboseError<&[u8]>> {
    map(tag("   "), drop)(input)
}

fn container_or_hole(input: &[u8]) -> IResult<&[u8], Option<Crate>, VerboseError<&[u8]>> {
    alt((map(container, Some), map(hole, |_| None)))(input)
}

fn container_line(input: &[u8]) -> IResult<&[u8], Vec<Option<Crate>>, VerboseError<&[u8]>> {
    terminated(many1(terminated(container_or_hole, opt(tag(" ")))), newline)(input)
}

fn index_line(input: &[u8]) -> IResult<&[u8], usize, VerboseError<&[u8]>> {
    let (input, v) = many1(terminated(
        delimited(char(' '), number, opt(char(' '))),
        alt((char(' '), newline)),
    ))(input)?;
    Ok((input, *v.last().unwrap()))
}

fn instruction(input: &[u8]) -> IResult<&[u8], Instruction, VerboseError<&[u8]>> {
    let (input, (num, from, to)) = terminated(
        tuple((
            preceded(tag("move "), number),
            preceded(tag(" from "), number::<_, usize, _>),
            preceded(tag(" to "), number::<_, usize, _>),
        )),
        opt(newline),
    )(input)?;
    Ok((
        input,
        Instruction {
            num,
            from: from - 1,
            to: to - 1,
        },
    ))
}

fn stacks_section(input: &[u8]) -> IResult<&[u8], Vec<Vec<Crate>>, VerboseError<&[u8]>> {
    let (input, container_lines) = many1(container_line)(input)?;
    let (input, num_containers) = index_line(input)?;
    let (input, _) = multispace1(input)?;

    let mut iters = container_lines
        .into_iter()
        .map(|n| n.into_iter())
        .collect::<Vec<_>>();

    let containers = (0..num_containers)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap_or(None))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok((input, containers))
}

fn stacks(input: &[u8]) -> IResult<&[u8], Stacks, VerboseError<&[u8]>> {
    let (input, stacks) = stacks_section(input)?; // Containers section

    let (input, instructions) = many1(instruction)(input)?; // Instruction section
    Ok((
        input,
        Stacks {
            stacks,
            instructions,
        },
    ))
}

impl Runner<String<9>, String<9>> for Day {
    type Input<'input> = Stacks;

    fn day() -> usize {
        5
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        let (_input, stacks) = all_consuming(stacks)(input.as_bytes())
            .finish()
            .map_err(|e| color_eyre::eyre::eyre!("{e:?}"))
            .expect("AoC input isn't broken");
        Ok(stacks)
    }

    fn part1(input: &Self::Input<'_>) -> Result<String<9>> {
        let mut stacks = input.stacks.clone();
        input
            .instructions
            .iter()
            .for_each(|&Instruction { num, from, to }| {
                let mut to_stack = std::mem::take(&mut stacks[to]);
                let new_len = stacks[from].len() - num;
                let drained = stacks[from][new_len..].iter().copied();
                to_stack.extend(drained.rev());
                stacks[from].truncate(new_len);
                let _ = std::mem::replace(&mut stacks[to], to_stack);
            });
        Ok(stacks
            .iter()
            .map(|l| l.last().unwrap())
            .map(|c| c.0)
            .collect())
    }

    fn part2(input: &Self::Input<'_>) -> Result<String<9>> {
        let mut stacks = input.stacks.clone();
        input
            .instructions
            .iter()
            .for_each(|&Instruction { num, from, to }| {
                // Need to get around mutable access rules. So we take this one out,
                // and replace it after adding stuff to it. This is an efficient pointer shift
                // Not actually cloning data. Or it shouldn't be
                let mut to_stack = std::mem::take(&mut stacks[to]);
                let new_len = stacks[from].len() - num;
                let drained = stacks[from][new_len..].iter().copied();
                to_stack.extend(drained);
                stacks[from].truncate(new_len);
                let _ = std::mem::replace(&mut stacks[to], to_stack);
            });
        Ok(stacks
            .iter()
            .map(|l| l.last().unwrap())
            .map(|c| c.0)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::{prod_case, sample_case};

    use super::*;

    sample_case! {
        sample1 =>
            input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        part1 = "CMZ";
        part2 = "MCD";
    }

    prod_case! {
        part1 = "CFFHVVHNC";
        part2 = "FSZWBPTBG";
    }
}
