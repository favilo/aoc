use anyhow::Result;
use nom::character::complete::anychar;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    character::complete::{alpha1, multispace0, space1},
    sequence::{terminated, tuple},
};

use crate::Runner;

pub struct Day02;

fn parse_line(input: &str) -> nom::IResult<&str, (usize, usize, u8, Vec<u8>)> {
    let (input, (s, _dash, e)) = terminated(tuple((digit1, tag("-"), digit1)), space1)(input)?;
    let (input, n) = terminated(anychar, tuple((tag(":"), space1)))(input)?;
    let (input, v) = terminated(alpha1, multispace0)(input)?;
    Ok((
        input,
        (
            s.parse().unwrap(),
            e.parse().unwrap(),
            n as u8,
            v.to_owned().into_bytes(),
        ),
    ))
}

type Pair = ((usize, usize), u8);
impl Runner for Day02 {
    type Input = Vec<(Pair, Vec<u8>)>;
    type Output = usize;

    fn day() -> usize {
        2
    }

    #[inline]
    fn get_input(input: &str) -> Result<Self::Input> {
        let lines = input.lines();
        let lines = lines.map(|line| {
            let (input, (start, end, c, pass)) = parse_line(line).unwrap();
            assert_eq!("", input);
            (((start, end), c), pass)
        });
        let lines = lines.collect::<Self::Input>();
        Ok(lines)
    }

    #[inline]
    fn part1(input: &Self::Input) -> Result<usize> {
        let correct = input.iter().filter(|&(((s, e), c), pass)| -> bool {
            (*s..=*e).contains(&bytecount::count(pass, *c))
        });
        Ok(correct.count())
    }

    #[inline]
    fn part2(input: &Self::Input) -> Result<usize> {
        let correct = input
            .iter()
            .map(|(((f, s), c), ref pass)| (pass[*f - 1] == *c as u8, pass[*s - 1] == *c as u8))
            .filter(|(a, b)| a != b);
        Ok(correct.count())
    }
}
