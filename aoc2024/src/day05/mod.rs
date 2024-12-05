use std::cmp::Ordering;

use miette::Result;
use multimap::MultiMap;
use winnow::{
    ascii::{dec_uint, line_ending},
    combinator::{repeat, separated, terminated},
    seq,
    stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial},
    PResult, Parser,
};

use crate::{errors::ToMiette, Runner};

pub struct Day;

#[derive(Debug, Default)]
pub struct Comparer {
    rules: MultiMap<usize, usize>,
}

impl Comparer {
    pub fn add_rule(&mut self, k: usize, v: usize) {
        self.rules.insert(k, v);
    }

    pub fn parse<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr,
    {
        let pairs: Vec<(usize, usize)> =
            repeat(1.., terminated(Self::rule, line_ending)).parse_next(input)?;
        let rules = MultiMap::from_iter(pairs);
        Ok(Self { rules })
    }

    fn rule<S>(input: &mut S) -> PResult<(usize, usize)>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr,
    {
        seq!(dec_uint, _: "|", dec_uint).parse_next(input)
    }

    fn compare(&self, a: usize, b: usize) -> bool {
        matches!(self.cmp(a, b), Ordering::Equal | Ordering::Less)
    }

    fn cmp(&self, a: usize, b: usize) -> Ordering {
        if self.rules.get_vec(&a).map_or(false, |v| v.contains(&b)) {
            Ordering::Less
        } else if self.rules.get_vec(&b).map_or(false, |v| v.contains(&a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

fn lists<S>(input: &mut S) -> PResult<Vec<Vec<usize>>>
where
    for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
    <S as Stream>::Token: AsChar + Clone,
    <S as Stream>::Slice: AsBStr,
{
    separated(
        1..,
        separated::<_, usize, Vec<usize>, _, _, _, _>(1.., dec_uint, ","),
        line_ending,
    )
    .parse_next(input)
}

fn puzzle<S>(input: &mut S) -> PResult<(Comparer, Vec<Vec<usize>>)>
where
    for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
    <S as Stream>::Token: AsChar + Clone,
    <S as Stream>::Slice: AsBStr,
{
    terminated(
        seq!(
            Comparer::parse,
            _: line_ending,
            lists
        ),
        line_ending,
    )
    .parse_next(input)
}

impl Runner for Day {
    type Input<'input> = (Comparer, Vec<Vec<usize>>);

    fn day() -> usize {
        5
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        puzzle.parse(input).to_miette()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let (comparer, lists) = input;

        Ok(lists
            .iter()
            .filter(|list| list.is_sorted_by(|&a, &b| comparer.compare(a, b)))
            .map(|list| list.get(list.len() / 2).unwrap())
            .sum())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let (comparer, lists) = input;

        let mut lists = lists.clone();
        Ok(lists
            .iter_mut()
            .filter(|list| !list.is_sorted_by(|&a, &b| comparer.compare(a, b)))
            .map(|list| {
                (*list).sort_by(|&a, &b| comparer.cmp(a, b));
                list
            })
            .map(|list| list.get(list.len() / 2).unwrap())
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                47|53\n\
                97|13\n\
                97|61\n\
                97|47\n\
                75|29\n\
                61|13\n\
                75|53\n\
                29|13\n\
                97|29\n\
                53|29\n\
                61|53\n\
                97|53\n\
                61|29\n\
                47|13\n\
                75|47\n\
                97|75\n\
                47|61\n\
                75|61\n\
                47|29\n\
                75|13\n\
                53|13\n\
                \n\
                75,47,61,53,29\n\
                97,61,53,29,13\n\
                75,29,13\n\
                75,97,47,61,53\n\
                61,13,29\n\
                97,13,75,29,47\n\
            ";
            part1 = 143;
            part2 = 123;
    }

    prod_case! {
        part1 = 4135;
        part2 = 201684;
    }
}
