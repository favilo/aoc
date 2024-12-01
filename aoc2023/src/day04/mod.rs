use std::collections::HashSet;

use color_eyre::Result;
use itertools::Itertools;
use winnow::{
    ascii::{digit1, space1},
    combinator::{separated, terminated},
    PResult, Parser,
};

use crate::Runner;

#[derive(Debug, Clone, Default)]
pub struct Card {
    winners: Vec<usize>,
    deck: Vec<usize>,
}

fn card(input: &mut &str) -> PResult<Card> {
    let _ = ("Card", space1, digit1, ":", space1).parse_next(input)?;
    let winners = separated(1.., digit1.try_map(str::parse::<usize>), space1).parse_next(input)?;
    let _ = (space1, "|", space1).parse_next(input)?;
    let deck = separated(1.., digit1.try_map(str::parse::<usize>), space1).parse_next(input)?;

    Ok(Card { winners, deck })
}

pub struct Day;

impl Runner for Day {
    type Input<'input> = Vec<Card>;

    fn day() -> usize {
        4
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(input
            .lines()
            .map(|line| card(&mut line.trim()).unwrap())
            .collect())
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .into_iter()
            .map(|card| {
                card.winners
                    .iter()
                    .copied()
                    .collect::<HashSet<_>>()
                    .intersection(&card.deck.iter().copied().collect())
                    .copied()
                    .collect_vec()
            })
            .map(|v| v.len())
            .filter(|v| *v > 0)
            .map(|l| 2usize.pow(l as u32 - 1))
            .sum())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut copies = vec![1; input.len()];
        for (idx, card) in input.into_iter().enumerate() {
            let wins = card
                .winners
                .iter()
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&card.deck.iter().copied().collect())
                .copied()
                .collect_vec();
            if wins.len() > 0 {
                (1..=wins.len()).for_each(|i| copies[idx + i] += copies[idx]);
            }
        }

        Ok(copies.iter().sum::<usize>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
            part1 = 13;
            part2 = 30;
    }

    prod_case! {
        part1 = 21485;
        part2 = 201684;
    }
}
