use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::{map, value},
    sequence::{delimited, terminated, tuple},
    IResult,
};

use crate::Runner;

#[derive(Debug, Clone, Copy)]
pub enum Movement {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn parse_movement(input: &str) -> IResult<&str, Movement> {
    let (input, ((), num)): (_, ((), Movement)) = delimited(
        multispace0,
        alt((
            tuple((
                value((), terminated(tag("forward"), multispace0)),
                map(digit1, |s: &str| Movement::Forward(s.parse().unwrap())),
            )),
            tuple((
                value((), terminated(tag("up"), multispace0)),
                map(digit1, |s: &str| Movement::Up(s.parse().unwrap())),
            )),
            tuple((
                value((), terminated(tag("down"), multispace0)),
                map(digit1, |s: &str| Movement::Down(s.parse().unwrap())),
            )),
        )),
        multispace0,
    )(input)?;
    Ok((input, num))
}

pub struct Day;
impl Runner for Day {
    type Input = Vec<Movement>;
    type Output = usize;

    fn day() -> usize {
        2
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .lines()
            .map(|line| {
                let (input, movement) = parse_movement(line).unwrap();
                assert_eq!("", input);
                movement
            })
            .collect())
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        let (h, v) = input.into_iter().fold((0, 0), |(mut h, mut v), m| {
            match m {
                Movement::Forward(m) => h += m,
                Movement::Down(m) => v += *m as isize,
                Movement::Up(m) => v -= *m as isize,
            };
            (h, v)
        });
        Ok(h * v as usize)
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let (h, v, _) = input
            .into_iter()
            .fold((0, 0, 0), |(mut h, mut v, mut aim), m| {
                match m {
                    Movement::Forward(m) => {
                        h += m;
                        v += aim * m;
                    }
                    Movement::Down(m) => aim += *m,
                    Movement::Up(m) => aim -= *m,
                };
                (h, v, aim)
            });
        Ok(h * v as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = r#"forward 5
                       down 5
                       forward 8
                       up 3
                       down 8
                       forward 2"#;

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(150, Day::part1(&input)?);
        assert_eq!(900, Day::part2(&input)?);
        Ok(())
    }
}
