use heapless::Vec;
use std::cmp::Ordering;

use color_eyre::Result;

use crate::{utils::trim_ascii_whitespace, Runner};

pub struct Day;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn from_hand(h: Hand) -> Self {
        match h {
            Hand::Rock => Self::Lose,
            Hand::Paper => Self::Draw,
            Hand::Scissors => Self::Win,
        }
    }

    fn score(&self) -> usize {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl Hand {
    fn score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn beat(self) -> Self {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (a, b) if a == b => Some(Ordering::Equal),
            (Hand::Rock, Hand::Scissors)
            | (Hand::Paper, Hand::Rock)
            | (Hand::Scissors, Hand::Paper) => Some(Ordering::Greater),
            _ => Some(Ordering::Less),
        }
    }
}

impl From<&[u8]> for Hand {
    fn from(s: &[u8]) -> Self {
        match s[0] {
            b'A' | b'X' => Hand::Rock,
            b'B' | b'Y' => Hand::Paper,
            b'C' | b'Z' => Hand::Scissors,
            c => panic!("Wrong letter {c:?}"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Round(Hand, Hand);

impl Round {
    fn score(&self) -> usize {
        self.outcome().score() + self.1.score()
    }

    fn outcome(&self) -> Outcome {
        let Self(other, mine) = self;
        match mine.partial_cmp(other) {
            Some(Ordering::Greater) => Outcome::Win,
            Some(Ordering::Equal) => Outcome::Draw,
            Some(Ordering::Less) => Outcome::Lose,
            None => unreachable!(),
        }
    }
}

impl From<&[u8]> for Round {
    fn from(s: &[u8]) -> Self {
        let s = trim_ascii_whitespace(s);
        Self(Hand::from(&s[0..1]), Hand::from(&s[2..3]))
    }
}

impl Runner for Day {
    type Input<'input> = Vec<Round, 2500>;

    fn day() -> usize {
        2
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(input.lines().map(str::as_bytes).map(Round::from).collect())
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input.iter().map(Round::score).sum())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .iter()
            .map(|r| (r.0, Outcome::from_hand(r.1)))
            .map(|(other, s)| {
                Round(
                    other,
                    match s {
                        Outcome::Lose => other.beat().beat(),
                        Outcome::Draw => other,
                        Outcome::Win => other.beat(),
                    },
                )
                .score()
            })
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::prod_case;

    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "\
            A Y
            B X
            C Z";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(15, Day::part1(&input)?);
        assert_eq!(12, Day::part2(&input)?);
        Ok(())
    }

    prod_case! {
        part1 = 14297;
        part2 = 10498;
    }
}
