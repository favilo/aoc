use anyhow::Result;
use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, one_of},
    combinator::map,
    multi::{many1, many_m_n},
    sequence::{terminated, tuple},
    IResult,
};

use crate::Runner;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Pair(char, char);

pub type Template = HashMap<Pair, usize>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Rule(Pair, char);

pub struct Day;

impl Runner for Day {
    type Input = (Template, Vec<Rule>);

    fn day() -> usize {
        14
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        let (input, temp) = template(input).unwrap();
        let (input, rules) = many1(rule)(input).unwrap();
        assert_eq!("", input);
        Ok((temp, rules))
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        apply(10, input)
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        apply(40, input)
    }
}

fn apply(
    n: usize,
    input: &(
        std::collections::HashMap<Pair, usize, std::hash::BuildHasherDefault<fxhash::FxHasher>>,
        Vec<Rule>,
    ),
) -> Result<usize, anyhow::Error> {
    let (map, rules) = input;
    let mut map = map.clone();
    (0..n).for_each(|_| {
        map = step(&map, rules);
    });
    let letters = map
        .iter()
        .map(|(p, _)| [p.0, p.1].into_iter())
        .flatten()
        .collect::<HashSet<_>>();
    let (min, max) = letters
        .iter()
        .map(|l| count(*l, &map))
        .fold((usize::MAX, 0), |(min, max), v| {
            (std::cmp::min(v, min), std::cmp::max(v, max))
        });
    Ok(max - min)
}

fn count(letter: char, template: &Template) -> usize {
    (template.iter().fold(0, |acum, (p, v)| {
        acum + if p.0 == letter && p.1 == letter {
            *v << 1
        } else if p.0 == letter || p.1 == letter {
            *v
        } else {
            0
        }
    }) as f64
        / 2.0)
        .ceil() as usize
}

fn step(template: &Template, rules: &[Rule]) -> Template {
    let mut new = Template::default();
    rules.into_iter().for_each(|&Rule(Pair(a, b), c): &Rule| {
        let o = template.get(&Pair(a, b));
        if o.is_none() {
            return;
        }
        // println!("Found pair: {:?}: {:?}", Pair(a, b), o);
        let o = o.unwrap();
        *new.entry(Pair(a, c)).or_insert(0) += o;
        *new.entry(Pair(c, b)).or_insert(0) += o;
    });
    new
}

fn template(input: &str) -> IResult<&str, Template> {
    let (input, temp) = terminated(alpha1, many1(newline))(input)?;
    let mut map = HashMap::default();
    temp.chars()
        .tuple_windows()
        .for_each(|(a, b)| *map.entry(Pair(a, b)).or_insert(0) += 1);
    Ok((input, map))
}

fn rule(input: &str) -> IResult<&str, Rule> {
    map(
        tuple((
            terminated(many_m_n(2, 2, letter), tag(" -> ")),
            terminated(letter, newline),
        )),
        |(ab, c): (Vec<char>, char)| -> Rule { Rule(Pair(ab[0], ab[1]), c) },
    )(input)
}

fn letter(input: &str) -> IResult<&str, char> {
    one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

        let input = Day::get_input(input)?;
        println!("{:?}", input);

        let (mut map, rules) = input.clone();
        (0..10).for_each(|_| {
            map = step(&map, &rules);
        });
        println!("\n{:?}", map);
        assert_eq!(1749, count('B', &map));
        assert_eq!(1588, Day::part1(&input)?);
        assert_eq!(2188189693529, Day::part2(&input)?);
        Ok(())
    }
}
