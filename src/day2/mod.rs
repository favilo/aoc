use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{complete::digit1, streaming::multispace0},
    combinator::map,
    sequence::{delimited, tuple},
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
    let (input, (dir, num)): (_, (&str, usize)) = tuple((
        delimited(
            multispace0,
            alt((tag("forward"), tag("up"), tag("down"))),
            multispace0,
        ),
        map(digit1, |s: &str| s.parse().unwrap()),
    ))(input)?;
    Ok((
        input,
        match dir {
            "forward" => Movement::Forward(num),
            "up" => Movement::Up(num),
            "down" => Movement::Down(num),
            &_ => unreachable!(),
        },
    ))
}

pub struct Day02;
impl Runner for Day02 {
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
        let (mut h, mut v) = (0, 0);
        input.into_iter().for_each(|m| match m {
            Movement::Forward(m) => h += m,
            Movement::Down(m) => v += *m as isize,
            Movement::Up(m) => v -= *m as isize,
        });
        Ok(h * v as usize)
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let (mut h, mut v, mut aim) = (0, 0, 0);
        input.into_iter().for_each(|m| match m {
            Movement::Forward(m) => {
                h += m;
                v += aim * m;
            }
            Movement::Down(m) => aim += *m,
            Movement::Up(m) => aim -= *m,
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

        let input = Day02::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(150, Day02::part1(&input)?);
        assert_eq!(900, Day02::part2(&input)?);
        Ok(())
    }
}
