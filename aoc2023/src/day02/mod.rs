use std::collections::BTreeMap;

use color_eyre::Result;
use itertools::any;
use winnow::{
    ascii::digit1,
    combinator::{alt, not, preceded, repeat, separated},
    token::take_while,
    PResult, Parser,
};

use crate::Runner;

pub struct Day;

#[derive(Debug, Default)]
pub struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

fn round(input: &mut &str) -> PResult<Round> {
    let colors: Vec<(usize, char, &str)> = separated(
        1..,
        (
            digit1.try_map(str::parse),
            ' ',
            alt(("green", "red", "blue")),
        ),
        ", ",
    )
    .parse_next(input)?;
    let color = colors
        .iter()
        .fold(Round::default(), |round, (num, _, color)| match *color {
            "red" => Round { red: *num, ..round },
            "green" => Round {
                green: *num,
                ..round
            },
            "blue" => Round {
                blue: *num,
                ..round
            },
            _ => unreachable!(),
        });
    Ok(color)
}

fn game(input: &mut &str) -> PResult<(usize, Vec<Round>)> {
    let game: usize = preceded("Game ", digit1.try_map(str::parse)).parse_next(input)?;
    let _ = ": ".parse_next(input)?;
    let mut rounds: Vec<&str> =
        separated(1.., take_while(1.., |c| c != ';').recognize(), "; ").parse_next(input)?;
    let rounds = rounds
        .iter_mut()
        .map(|r| round(r).unwrap())
        .collect::<Vec<_>>();
    Ok((game, rounds))
}

impl Runner for Day {
    type Input<'input> = BTreeMap<usize, Vec<Round>>;

    fn day() -> usize {
        2
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(input
            .lines()
            .map(|mut line| game.parse_next(&mut line).unwrap())
            .collect())
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let red = 12;
        let green = 13;
        let blue = 14;
        Ok(input
            .iter()
            .filter(|(_, rounds)| {
                !any(rounds.iter(), |r| {
                    r.red > red || r.green > green || r.blue > blue
                })
            })
            // .map(|g| dbg!(g))
            .map(|(game, _)| *game)
            .sum::<usize>())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input.iter().map(|(_, rounds)| power(rounds)).sum::<usize>())
    }
}

fn power(input: &[Round]) -> usize {
    let round = input.iter().fold(Round::default(), |acc, r| Round {
        red: acc.red.max(r.red),
        green: acc.green.max(r.green),
        blue: acc.blue.max(r.blue),
    });
    round.red * round.green * round.blue
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
            part1 = 8;
            part2 = 2286;
    }

    prod_case! {
        part1 = 2505;
        part2 = 70265;
    }
}
