use std::ops::{ControlFlow, RangeInclusive};

use itertools::Itertools;
use miette::{IntoDiagnostic, Result};

use aoc_utils::{
    math::{digit_count, digits},
    Runner,
};

pub struct Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Passing {
    Passes(usize),
    Fails(usize),
}

impl Passing {
    fn next(&self) -> usize {
        let (Passing::Passes(n) | Passing::Fails(n)) = self;
        *n
    }

    fn is_passing(&self) -> bool {
        matches!(self, Passing::Passes(_))
    }
}

fn next_to_check(n: usize) -> usize {
    let mut first_bad = None;
    let Some(first_digit) = digits(n).last().map(|d| d as usize) else {
        panic!("No digits found");
    };
    log::debug!("Finding next after: {n}");
    digits(n)
        .rev()
        .tuple_windows()
        .map(|(a, b)| {
            if let Some(bad) = first_bad {
                bad
            } else if a > b {
                first_bad = Some(a);
                a
            } else {
                b
            }
        })
        .fold(first_digit, |acc, d| acc * 10 + d as usize)
}

fn matches(n: usize) -> Passing {
    log::debug!("Checking {n}");
    match digits(n)
        .rev()
        .try_fold((None, false), |(last, double), d| {
            log::debug!("{d}: {last:?} double: {double}");
            if let Some(last) = last {
                if last > d {
                    log::debug!("digits inverted, {last} > {d}");
                    return ControlFlow::Break(next_to_check(n));
                }
            }

            ControlFlow::Continue((Some(d), double || Some(d) == last))
        }) {
        ControlFlow::Continue((_, true)) => Passing::Passes(n + 1),
        ControlFlow::Continue((_, false)) => Passing::Fails(n + 1),
        ControlFlow::Break(next) => Passing::Fails(next),
    }
}

fn matches_advanced(n: usize) -> Passing {
    assert_eq!(digit_count(n), 6);
    let next = next_to_check(n + 1);
    match digits(n)
        .rev()
        .try_fold(heapless::Vec::<_, 6>::new(), |mut runs, d| {
            if let Some((last, c)) = runs.last_mut() {
                if *last > d {
                    return ControlFlow::Break(());
                }

                if d == *last {
                    *c += 1;
                } else {
                    runs.push((d, 1)).unwrap();
                }
            } else {
                runs.push((d, 1)).unwrap();
            }

            ControlFlow::Continue(runs)
        }) {
        ControlFlow::Continue(runs) => {
            if runs.into_iter().filter(|(_, c)| *c == 2).count() > 0 {
                Passing::Passes(next)
            } else {
                Passing::Fails(next)
            }
        }
        ControlFlow::Break(()) => Passing::Fails(next),
    }
}

impl Runner for Day {
    type Input<'input> = RangeInclusive<usize>;

    #[rustfmt::skip]
    fn day() -> usize {
        4
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        let Some((start, end)) = input.trim().split_once('-') else {
            return Err(miette::miette!("Invalid input format"));
        };
        Ok(start.parse().into_diagnostic()?..=end.parse().into_diagnostic()?)
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut last = *input.start();
        let mut count = 0;
        while last <= *input.end() {
            let next = matches(last);
            if next.is_passing() {
                count += 1;
            }
            last = next.next();
        }
        Ok(count)
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut last = *input.start();
        let mut count = 0;
        while last <= *input.end() {
            let next = matches_advanced(last);
            if next.is_passing() {
                log::debug!("Passing: {last}");
                count += 1;
            }
            last = next.next();
            log::debug!("next: {last}");
        }
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::prod_case;

    // No sample case for this one

    prod_case! {
        part1 = 2779;
        part2 = 1972;
    }
}
