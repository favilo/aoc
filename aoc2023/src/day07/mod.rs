use std::cmp::{Ordering, Reverse};

use color_eyre::Result;
use counter::Counter;

use crate::Runner;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!(),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Type {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Hand([Card; 5]);

impl Hand {
    fn get_type(self) -> Type {
        let mut counters = self.0.into_iter().collect::<Counter<_>>();

        let jokers = counters[&Card::Joker];
        counters[&Card::Joker] = 0;

        let ordered = counters.most_common_ordered();
        if ordered[0].1 + jokers == 5 {
            return Type::FiveOfAKind;
        }
        if ordered[0].1 + jokers == 4 {
            return Type::FourOfAKind;
        }
        if ordered[0].1 + jokers == 3 {
            if ordered[1].1 == 2 {
                return Type::FullHouse;
            } else {
                return Type::ThreeOfAKind;
            }
        }
        if ordered[0].1 + jokers == 2 {
            if ordered[1].1 == 2 {
                return Type::TwoPair;
            } else {
                return Type::OnePair;
            }
        }
        return Type::HighCard;
    }

    fn to_joker(&mut self) {
        self.0.iter_mut().for_each(|c| match *c {
            Card::Jack => *c = Card::Joker,
            _ => {}
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_type().cmp(&other.get_type()) {
            Ordering::Equal => {
                self.0
                    .iter()
                    .zip(other.0.iter())
                    .fold(Ordering::Equal, |acc, (s, o)| match acc {
                        Ordering::Equal => s.cmp(&o),
                        other => other,
                    })
            }
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day;

impl Runner for Day {
    type Input<'input> = Vec<(Hand, usize)>;

    fn day() -> usize {
        7
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(input
            .lines()
            .map(|line| line.split_once(" ").unwrap())
            .map(|(a, b)| {
                (
                    Hand(
                        a.chars()
                            .map(Card::from_char)
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap(),
                    ),
                    b.parse().unwrap(),
                )
            })
            .collect())
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut input = input.to_owned();
        input.sort_by_key(|t| t.0);
        Ok(input
            .into_iter()
            .enumerate()
            .map(|(idx, (_, bid))| (idx + 1) * bid)
            .sum::<usize>())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut input = input.to_owned();
        input.iter_mut().for_each(|(h, _)| h.to_joker());
        input.sort_by_key(|t| t.0);
        Ok(input
            .into_iter()
            .enumerate()
            .map(|(idx, (_, bid))| (idx + 1) * bid)
            .sum::<usize>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                32T3K 765\n\
                T55J5 684\n\
                KK677 28\n\
                KTJJT 220\n\
                QQQJA 483\n\
            ";
            part1 = 6440;
            part2 = 5905;
    }

    prod_case! {
        part1 = 250453939;
        part2 = 248652697;
    }
}
