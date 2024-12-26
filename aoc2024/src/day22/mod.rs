use std::sync::atomic::{AtomicUsize, Ordering};

use itertools::{iterate, Itertools};
use miette::{IntoDiagnostic, Result};

use crate::Runner;

pub struct Day;

static LIMIT: AtomicUsize = AtomicUsize::new(2000);

fn mix(secret: &mut usize, input: usize) {
    *secret ^= input;
}

fn prune(secret: &mut usize) {
    *secret %= 16777216;
}

fn step_1(secret: &mut usize) {
    mix(secret, *secret * 64);
    prune(secret);
}

fn step_2(secret: &mut usize) {
    mix(secret, *secret >> 5);
    prune(secret);
}

fn step_3(secret: &mut usize) {
    mix(secret, *secret << 11);
    prune(secret);
}

fn next_secret(secret: &usize) -> usize {
    let mut secret = *secret;
    step_1(&mut secret);
    step_2(&mut secret);
    step_3(&mut secret);
    secret
}

fn price(secret: usize) -> u8 {
    (secret % 10) as u8
}

impl Runner for Day {
    type Input<'input> = Vec<usize>;

    #[rustfmt::skip]
    fn day() -> usize {
        22
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        input
            .lines()
            .map(|line| line.parse::<usize>())
            .collect::<Result<Self::Input<'_>, _>>()
            .into_diagnostic()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .iter()
            .filter_map(|&secret| iterate(secret, next_secret).nth(2000))
            .sum())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut cache = vec![(usize::MAX, 0); 19usize.pow(4)];

        input.iter().copied().enumerate().for_each(|(id, secret)| {
            let prices = prices(secret).take(LIMIT.load(Ordering::Relaxed));
            let deltas = price_changes(prices.clone());
            let hashes = hashes(subsequences(deltas.clone()));
            prices.skip(4).zip(hashes).for_each(|(price, hash)| {
                let entry = &mut cache[hash];
                if entry.0 != id {
                    *entry = (id, (price as usize) + entry.1);
                }
            });
        });

        Ok(cache.iter().copied().map(|(_, price)| price).max().unwrap())
    }
}

fn subsequences(deltas: impl IntoIterator<Item = u8>) -> impl Iterator<Item = (u8, u8, u8, u8)> {
    deltas.into_iter().tuple_windows::<(u8, u8, u8, u8)>()
}

fn hashes(subsequences: impl Iterator<Item = (u8, u8, u8, u8)>) -> impl Iterator<Item = usize> {
    subsequences.map(|(a, b, c, d)| {
        ((a as usize & 0xff) * 19 * 19 * 19)
            + ((b as usize & 0xff) * 19 * 19)
            + ((c as usize & 0xff) * 19)
            + (d as usize & 0xff)
    })
}

fn prices(secret: usize) -> impl Iterator<Item = u8> + Clone {
    iterate(secret, next_secret).map(price)
}

fn price_changes(prices: impl Iterator<Item = u8> + Clone) -> impl Iterator<Item = u8> + Clone {
    prices
        .tuple_windows()
        .map(|(a, b)| b as i8 - a as i8)
        .inspect(|i| debug_assert!(*i >= -9))
        .map(|i| (i + 9) as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            preamble = {
                // LIMIT.store(10, Ordering::Relaxed);
            };
            input1 = indoc::indoc! {"
                1
                10
                100
                2024
            "};
            part1 = 37327623;
            input2 = indoc::indoc! {"
                1
                2
                3
                2024
            "};
            part2 = 23;
    }

    prod_case! {
        part1 = 0;
        part2 = 0;
    }

    #[test]
    fn prune_100000000() {
        let mut secret = 100000000;
        prune(&mut secret);
        assert_eq!(secret, 16113920);
    }

    #[test]
    fn mix_42() {
        let mut secret = 42;
        mix(&mut secret, 15);
        assert_eq!(secret, 37);
    }

    #[test]
    fn next_123() {
        let secret = next_secret(&123);
        assert_eq!(secret, 15887950);
    }

    #[test]
    fn next_ten_123() {
        let secrets = iterate(123, next_secret)
            .skip(1)
            .take(10)
            .collect::<Vec<_>>();
        assert_eq!(
            secrets,
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254,
            ]
        );
    }

    #[test]
    #[allow(clippy::identity_op)]
    fn price_changes_123() {
        assert_eq!(
            price_changes(prices(123)).take(9).collect_vec(),
            vec![
                9 - 3,
                9 + 6,
                9 - 1,
                9 - 1,
                9 + 0,
                9 + 2,
                9 - 2,
                9 + 0,
                9 - 2,
            ]
        );
    }

    #[test]
    fn price_changes_subsequence() {
        let sequence = (9 - 2, 9 + 1, 9 - 1, 9 + 3);
        let changes = price_changes(prices(1)).take(2000).collect_vec();
        assert!(subsequences(changes).any(|seq| seq == sequence));

        let changes = price_changes(prices(2)).take(2000).collect_vec();
        assert!(subsequences(changes).any(|seq| seq == sequence));

        let changes = price_changes(prices(3)).take(2000).collect_vec();
        assert!(!subsequences(changes).any(|seq| seq == sequence));

        let changes = price_changes(prices(2024)).take(2000).collect_vec();
        assert!(subsequences(changes).any(|seq| seq == sequence));
    }
}
