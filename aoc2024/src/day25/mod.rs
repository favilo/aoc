use itertools::iproduct;
use miette::Result;
use winnow::{
    ascii::line_ending,
    combinator::{alt, opt, repeat},
    seq,
    stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial},
    PResult, Parser,
};

use crate::errors::ToMiette;
use aoc_utils::Runner;

pub struct Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lock {
    Key(u8, u8, u8, u8, u8),
    Lock(u8, u8, u8, u8, u8),
}

impl Lock {
    fn key_from_array(bits: [u8; 5]) -> Self {
        Self::Key(bits[0], bits[1], bits[2], bits[3], bits[4])
    }

    fn lock_from_array(bits: [u8; 5]) -> Self {
        Self::Lock(bits[0], bits[1], bits[2], bits[3], bits[4])
    }

    fn parser<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr,
    {
        alt((Self::lock, Self::key)).parse_next(input)
    }

    fn key<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr,
    {
        seq!(
            _: ".....",
            _: line_ending,
            repeat(6..7, Self::bit_line),
            _: opt(line_ending),
        )
        .map(|(bits,): (Vec<[bool; 5]>,)| {
            Lock::key_from_array(
                bits.into_iter()
                    .enumerate()
                    .fold([None::<u8>; 5], |mut acc, (idx, these)| {
                        these.iter().enumerate().for_each(|(i, bit)| {
                            if acc[i].is_none() && *bit {
                                acc[i] = Some(5 - idx as u8);
                            }
                        });
                        acc
                    })
                    .map(Option::unwrap),
            )
        })
        .parse_next(input)
    }

    #[allow(clippy::self_named_constructors)]
    fn lock<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr,
    {
        seq! {
            _: "#####",
            _: line_ending,
            repeat(6..7, Self::bit_line),
            _: opt(line_ending),
        }
        .map(|(bits,): (Vec<[bool; 5]>,)| {
            Lock::lock_from_array(
                bits.into_iter()
                    .enumerate()
                    .fold([None::<u8>; 5], |mut acc, (idx, these)| {
                        these.iter().enumerate().for_each(|(i, bit)| {
                            if acc[i].is_none() && !bit {
                                acc[i] = Some(idx as u8);
                            }
                        });
                        acc
                    })
                    .map(Option::unwrap),
            )
        })
        .parse_next(input)
    }

    fn bit_line<S>(input: &mut S) -> PResult<[bool; 5]>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr,
    {
        seq!(
            repeat(5..6, alt(("#".map(|_| true), ".".map(|_| false)))),
            _: line_ending,
        )
        .map(|(bits,): (Vec<bool>,)| [bits[0], bits[1], bits[2], bits[3], bits[4]])
        .parse_next(input)
    }

    fn fits(self, other: Self) -> bool {
        match (self, other) {
            (Self::Key(ak, bk, ck, dk, ek), Self::Lock(al, bl, cl, dl, el))
            | (Self::Lock(al, bl, cl, dl, el), Self::Key(ak, bk, ck, dk, ek)) => {
                al + ak < 6 && bl + bk < 6 && cl + ck < 6 && dl + dk < 6 && el + ek < 6
            }
            _ => false,
        }
    }
}

impl Runner for Day {
    type Input<'input> = Vec<Lock>;

    #[rustfmt::skip]
    fn day() -> usize {
        25
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        repeat(1.., Lock::parser).parse(input).to_miette()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let keys = input
            .iter()
            .copied()
            .filter(|lock| matches!(lock, Lock::Key(_, _, _, _, _)));
        let locks = input
            .iter()
            .copied()
            .filter(|lock| matches!(lock, Lock::Lock(_, _, _, _, _)));

        let count = iproduct!(keys, locks)
            .filter(|(key, lock)| key.fits(*lock))
            .count();
        Ok(count)
    }

    fn part2(_input: &Self::Input<'_>) -> Result<usize> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = indoc::indoc! {"
                #####
                .####
                .####
                .####
                .#.#.
                .#...
                .....

                #####
                ##.##
                .#.##
                ...##
                ...#.
                ...#.
                .....

                .....
                #....
                #....
                #...#
                #.#.#
                #.###
                #####

                .....
                .....
                #.#..
                ###..
                ###.#
                ###.#
                #####

                .....
                .....
                .....
                #....
                #.#..
                #.#.#
                #####
            "};
            part1 = 3;
            part2 = 0;
    }

    prod_case! {
        part1 = 3284;
        part2 = 0;
    }
}
