use std::{cmp::Ordering, fmt::Debug, iter::once};

use color_eyre::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, u64},
    combinator::{all_consuming, map, opt},
    error::VerboseError,
    multi::{many0, many1},
    sequence::{delimited, terminated, tuple},
    IResult,
};
use nom_supreme::ParserExt;

use crate::Runner;

pub struct Day;

#[derive(Clone, Eq, PartialEq)]
pub enum Entry {
    List(Vec<Entry>),
    Int(usize),
}

impl Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List(list) => {
                write!(f, "[")?;
                write!(
                    f,
                    "{}",
                    list.iter().map(|entry| format!("{:?}", entry)).join(",")
                )?;
                write!(f, "]")
            }
            Self::Int(int) => write!(f, "{}", int),
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            // Integers, just compare them
            (Entry::Int(a), Entry::Int(b)) => a.partial_cmp(b),
            // Lists, compare element-wise, continue if equal so far
            (Entry::List(a), Entry::List(b)) => a
                .iter()
                .zip_longest(b)
                .map(|z| match z {
                    itertools::EitherOrBoth::Both(a, b) => a.partial_cmp(b),
                    itertools::EitherOrBoth::Left(_) => Some(Ordering::Greater),
                    itertools::EitherOrBoth::Right(_) => Some(Ordering::Less),
                })
                .find(|ord| ord != &Some(Ordering::Equal))
                .unwrap_or(Some(Ordering::Equal)),
            (list @ Entry::List(_), int @ Entry::Int(_)) => {
                list.partial_cmp(&Entry::List(vec![int.clone()]))
            }
            (int @ Entry::Int(_), list @ Entry::List(_)) => {
                Entry::List(vec![int.clone()]).partial_cmp(list)
            }
        }
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Packet(Entry);

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn number(input: &str) -> IResult<&str, Entry, VerboseError<&str>> {
    map(u64, |u| Entry::Int(u as usize))(input)
}

fn list(input: &str) -> IResult<&str, Entry, VerboseError<&str>> {
    delimited(
        tag("[").context("opening bracket"),
        map(
            many0(terminated(entry.context("sub-entry"), opt(tag(",")))),
            Entry::List,
        ),
        tag("]").context("closing bracket"),
    )(input)
}

fn entry(input: &str) -> IResult<&str, Entry, VerboseError<&str>> {
    alt((number.context("number"), list.context("list")))(input)
}

fn packet(input: &str) -> IResult<&str, Packet, VerboseError<&str>> {
    terminated(map(entry.context("entry"), Packet), opt(line_ending))(input)
}

fn pair(input: &str) -> IResult<&str, (Packet, Packet), VerboseError<&str>> {
    terminated(tuple((packet, packet)), opt(line_ending))(input)
}

impl Runner for Day {
    type Input<'input> = Vec<(Packet, Packet)>;

    fn day() -> usize {
        13
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(all_consuming(many1(pair))(input).unwrap().1)
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .iter()
            .enumerate()
            .filter_map(|(i, (a, b))| (a <= b).then_some(i + 1))
            .sum())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let packet_2 = packet("[[2]]").unwrap().1;
        let packet_6 = packet("[[6]]").unwrap().1;
        let answer = [packet_2, packet_6]
            .iter()
            .enumerate()
            .map(|(i, packet)| {
                input
                    .iter()
                    .flat_map(|(a, b)| once(a).chain(once(b)))
                    .filter(|&a| a < packet)
                    .count()
                    + 1
                    + i
            })
            .product();
        Ok(answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                [1,1,3,1,1]\n\
                [1,1,5,1,1]\n\
                \n\
                [[1],[2,3,4]]\n\
                [[1],4]\n\
                \n\
                [9]\n\
                [[8,7,6]]\n\
                \n\
                [[4,4],4,4]\n\
                [[4,4],4,4,4]\n\
                \n\
                [7,7,7,7]\n\
                [7,7,7]\n\
                \n\
                []\n\
                [3]\n\
                \n\
                [[[]]]\n\
                [[]]\n\
                \n\
                [1,[2,[3,[4,[5,6,7]]]],8,9]\n\
                [1,[2,[3,[4,[5,6,0]]]],8,9]";
            part1 = 13;
            part2 = 140;
    }

    prod_case! {
        part1 = 5675;
        part2 = 20383;
    }

    #[test]
    fn packet_ordering() {
        assert!(packet("[1,1,3,1,1]").unwrap().1 < packet("[1,1,5,1,1]").unwrap().1);
        assert!(packet("[9]").unwrap().1 > packet("[[8,7,6]]").unwrap().1);
        assert!(packet("[7,7,7,7]").unwrap().1 > packet("[7,7,7]").unwrap().1);
        assert!(packet("[]").unwrap().1 < packet("[3]").unwrap().1);
        assert!(packet("[[[]]]").unwrap().1 > packet("[[]]").unwrap().1);
    }
}
