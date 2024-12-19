use std::{borrow::Cow, hash::Hash};

use cached::{proc_macro::cached, Cached};
use miette::Result;
use trie_rs::Trie;
use winnow::{
    ascii::{alpha1, line_ending, space0},
    combinator::{opt, repeat, separated, terminated},
    error::{StrContext, StrContextValue},
    stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial},
    PResult, Parser,
};

use crate::{errors::ToMiette, Runner};

pub struct Day;

#[derive(Clone)]
pub struct Onsen<'input> {
    goals: Vec<Cow<'input, str>>,
    trie: Trie<u8>,
}

impl std::fmt::Debug for Onsen<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Onsen")
            .field("reversed_goals", &self.goals)
            .field_with("trie", |f| {
                f.debug_set()
                    .entries(self.trie.iter().collect::<Vec<String>>())
                    .finish()
            })
            .finish()
    }
}

impl<'input> Onsen<'input> {
    pub fn parser<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream<Slice = &'input str> + StreamIsPartial + Compare<&'a str> + 'input,
        <S as Stream>::Token: AsChar + Copy,
        <S as Stream>::Slice: Eq + Hash + AsBStr + 'input,
    {
        (
            Self::towels.context(StrContext::Label("towels")),
            line_ending,
            Self::goal.context(StrContext::Label("patterns")),
        )
            .map(|(towels, _, goals)| {
                // This actually saves like 30 ms on my machine from doing it forward
                let reversed_goals = goals
                    .iter()
                    .map(|goal| goal.chars().rev().collect::<Cow<'input, str>>())
                    .collect::<Vec<_>>();
                let reverse = Trie::from_iter(
                    &towels
                        .iter()
                        .map(|towel| towel.chars().rev().collect::<String>())
                        .collect::<Vec<_>>(),
                );
                Self {
                    trie: reverse,
                    goals: reversed_goals,
                }
            })
            .parse_next(input)
    }

    fn towels<S>(input: &mut S) -> PResult<Vec<&'input str>>
    where
        for<'a> S: Stream<Slice = &'input str> + StreamIsPartial + Compare<&'a str> + 'input,
        <S as Stream>::Token: AsChar + Copy,
        <S as Stream>::Slice: Eq + Hash + AsBStr + 'input,
    {
        terminated(
            separated(
                1..,
                alpha1.context(StrContext::Label("towel")),
                (",", space0),
            ),
            line_ending,
        )
        .parse_next(input)
    }

    fn goal<S>(input: &mut S) -> PResult<Vec<Cow<'input, str>>>
    where
        for<'a> S: Stream<Slice = &'input str> + StreamIsPartial + Compare<&'a str> + 'input,
        <S as Stream>::Token: AsChar + Copy,
        <S as Stream>::Slice: Eq + Hash + AsBStr + 'input,
    {
        repeat(
            1..,
            terminated(
                alpha1
                    .context(StrContext::Label("goal"))
                    .map(|s: &str| Cow::Borrowed(s)),
                opt(line_ending).context(StrContext::Label("new line")),
            ),
        )
        .context(StrContext::Expected(StrContextValue::Description("goals")))
        .parse_next(input)
    }
}

#[cached(key = "String", convert = r#"{ goal.to_string() }"#)]
pub fn is_match(onsen: &Onsen<'_>, goal: &str) -> bool {
    if goal.is_empty() {
        return true;
    }

    log::debug!("Searching for goal: {goal}");

    if onsen.trie.exact_match(goal) {
        log::debug!("Found exact match: {goal}");
        return true;
    }

    let prefixes = onsen.trie.common_prefix_search(goal);
    prefixes
        .inspect(|prefix| {
            log::debug!("common: {prefix}");
            log::debug!("goal: {goal}");
        })
        .any(|prefix: String| {
            let left = &goal[prefix.len()..];
            is_match(onsen, left)
        })
}

#[cached(key = "String", convert = r#"{ goal.to_string() }"#)]
pub fn count_matches(onsen: &Onsen<'_>, goal: &str /* , shift: usize */) -> usize {
    // let spaces = String::from_iter(vec![' '; shift]);
    let spaces = String::new();
    if goal.is_empty() {
        return 1;
    }

    log::debug!("{spaces}Searching for goal: {goal}");

    let prefixes = onsen.trie.common_prefix_search(goal);
    log::debug!("{spaces}prefixes: {:?}", Vec::from_iter(prefixes.clone()));
    prefixes
        .map(|prefix: String| {
            log::debug!("{spaces}common: {prefix}");
            log::debug!("{spaces}goal: {goal}");
            let left = &goal[prefix.len()..];
            count_matches(onsen, left)
        })
        .inspect(|count| {
            log::debug!("{spaces}count: {count}");
        })
        .sum()
}

impl Runner for Day {
    type Input<'input> = Onsen<'input>;

    #[rustfmt::skip]
    fn day() -> usize {
        19
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Onsen::parser.parse(input).to_miette()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        // Clear cache so we aren't cheating in our benchmarks
        IS_MATCH.lock().unwrap().cache_clear();
        Ok(input
            .goals
            .iter()
            .filter(|goal| is_match(input, goal))
            .count())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        // Clear cache so we aren't cheating in our benchmarks
        COUNT_MATCHES.lock().unwrap().cache_clear();
        Ok(input
            .goals
            .iter()
            .map(|goal| count_matches(input, goal))
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = indoc::indoc! {"
                r, wr, b, g, bwu, rb, gb, br

                brwrr
                bggr
                gbbr
                rrbgbr
                ubwu
                bwurrg
                brgr
                bbrgwb
            "};
            part1 = 6;
            part2 = 16;
    }

    prod_case! {
        part1 = 313;
        part2 = 666491493769758;
    }
}
