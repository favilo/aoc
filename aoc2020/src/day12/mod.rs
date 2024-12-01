use anyhow::Result;
use nom::{
    branch::alt,
    character::complete::{char, digit1, multispace0},
    combinator::value,
    sequence::tuple,
    IResult,
};

use crate::Runner;

pub struct Day12;

type Position = (isize, isize);

fn rotate_pos(pos: Position, angle: isize) -> Position {
    match angle {
        0 => pos,
        90 => (-pos.1, pos.0),
        180 => (-pos.0, -pos.1),
        270 => (pos.1, -pos.0),
        _ => panic!("at the disco"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    East,
    South,
    North,
    West,
}

impl Direction {
    fn vector(&self) -> Position {
        match self {
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::North => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    fn degrees(&self) -> isize {
        match self {
            Direction::East => 0,
            Direction::South => 270,
            Direction::North => 90,
            Direction::West => 180,
        }
    }

    fn dir(angle: isize) -> Self {
        let angle = if angle < 0 { angle + 360 } else { angle };
        let angle = angle % 360;
        match angle {
            0 => Self::East,
            90 => Self::North,
            180 => Self::West,
            270 => Self::South,
            _ => panic!("Bad angle"),
        }
    }

    fn add(&self, angle: isize) -> Self {
        Self::dir(angle + self.degrees())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    F(isize),
    L(isize),
    R(isize),
    N(isize),
    E(isize),
    S(isize),
    W(isize),
}

impl Instruction {
    fn from(c: char, a: isize) -> Self {
        match c {
            'F' => Self::F(a),
            'L' => Self::L(a),
            'R' => Self::R(a),
            'N' => Self::N(a),
            'E' => Self::E(a),
            'S' => Self::S(a),
            'W' => Self::W(a),
            _ => unreachable!(),
        }
    }
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, c): (&str, char) = alt((
        char('F'),
        char('L'),
        char('R'),
        char('N'),
        char('E'),
        char('S'),
        char('W'),
    ))(input)?;
    let (_, (num, ())) = tuple((digit1, value((), multispace0)))(input)?;

    Ok(("", Instruction::from(c, num.parse().unwrap())))
}

impl Runner for Day12 {
    type Input = Vec<Instruction>;
    type Output = usize;

    fn day() -> usize {
        12
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .lines()
            .map(parse_instruction)
            .map(Result::unwrap)
            .map(|t| t.1)
            .collect())
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        let mut position = (0, 0);
        let mut dir = Direction::East;

        input.iter().for_each(|&i| match i {
            Instruction::F(a) => {
                position = (
                    position.0 + a * dir.vector().0,
                    position.1 + a * dir.vector().1,
                );
            }
            Instruction::L(d) => dir = dir.add(d),
            Instruction::R(d) => dir = dir.add(-d),
            Instruction::N(a) => position.1 += a,
            Instruction::E(a) => position.0 += a,
            Instruction::S(a) => position.1 -= a,
            Instruction::W(a) => position.0 -= a,
        });
        Ok((position.0.abs() + position.1.abs()) as usize)
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let mut position = (0, 0);
        let mut waypoint = (10, 1);

        input.iter().for_each(|&i| match i {
            Instruction::F(a) => {
                position = (position.0 + a * waypoint.0, position.1 + a * waypoint.1);
            }
            Instruction::L(d) => {
                let d = Direction::dir(d).degrees();
                waypoint = rotate_pos(waypoint, d);
            }
            Instruction::R(d) => {
                let d = Direction::dir(-d).degrees();
                waypoint = rotate_pos(waypoint, d);
            }
            Instruction::N(a) => waypoint.1 += a,
            Instruction::E(a) => waypoint.0 += a,
            Instruction::S(a) => waypoint.1 -= a,
            Instruction::W(a) => waypoint.0 -= a,
        });

        Ok((position.0.abs() + position.1.abs()) as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "F10\n\
                     N3\n\
                     F7\n\
                     R90\n\
                     F11";

        let input = Day12::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(25, Day12::part1(&input)?);
        assert_eq!(286, Day12::part2(&input)?);
        Ok(())
    }
}
