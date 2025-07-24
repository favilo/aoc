use miette::Result;

use aoc_utils::{errors::ToMiette, Runner};
use winnow::{
    ascii::{alpha1, line_ending},
    combinator::{repeat, terminated},
    seq,
    stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial},
    PResult, Parser,
};

pub struct Day;

fn orbit<'input, S>(input: &'input mut S) -> PResult<(&'input str, &'input str)>
where
    for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
    <S as Stream>::Token: AsChar + Clone,
    <S as Stream>::Slice: AsRef<str>,
{
    seq! {alpha1, _: ")", alpha1}
        .map(
            |(orbitee, orbiter): (<S as Stream>::Slice, <S as Stream>::Slice)| {
                (orbitee.as_ref(), orbiter.as_ref())
            },
        )
        .parse_next(input)
}

impl Runner for Day {
    type Input<'input> = Vec<(&'input str, &'input str)>;

    #[rustfmt::skip]
    fn day() -> usize {
        6
    }

    fn get_input<'input>(input: &'input str) -> Result<Self::Input<'input>> {
        repeat(1.., terminated(orbit, line_ending))
            .parse(input)
            .to_miette()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        todo!()
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = indoc::indoc! {"
                COM)B
                B)C
                C)D
                D)E
                E)F
                B)G
                G)H
                D)I
                E)J
                J)K
                K)L
            "};
            part1 = 42;
            part2 = 0;
    }

    prod_case! {
        part1 = 0;
        part2 = 0;
    }
}
