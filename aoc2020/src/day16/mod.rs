use std::hash::BuildHasherDefault;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::RangeInclusive,
};

use anyhow::Result;
use hashers::fx_hash::FxHasher;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    bytes::complete::take_until,
    character::complete::digit1,
    character::complete::multispace0,
    combinator::{opt, value},
    multi::{fold_many1, many1},
    sequence::{terminated, tuple},
    IResult,
};

use crate::Runner;

type DefaultHasher = BuildHasherDefault<FxHasher>;
type MyMap<K, V> = HashMap<K, V, DefaultHasher>;
type MySet<V> = HashSet<V, DefaultHasher>;

#[derive(Debug, Clone)]
pub struct Tickets {
    fields: MyMap<String, Vec<RangeInclusive<usize>>>,
    yours: Vec<usize>,
    others: Vec<Vec<usize>>,
}

pub struct Day16;

fn parse_fields(input: &str) -> IResult<&str, (String, Vec<RangeInclusive<usize>>)> {
    let (input, field_name) = terminated(take_until(": "), tag(": "))(input)?;
    let (input, ranges) = terminated(
        many1(terminated(
            tuple((digit1, tag("-"), digit1)),
            opt(tag(" or ")),
        )),
        multispace0,
    )(input)?;
    let ranges = ranges
        .iter()
        .map(|t| (t.0.parse().unwrap()..=t.2.parse().unwrap()))
        .collect();
    Ok((input, (field_name.into(), ranges)))
}

fn ticket(input: &str) -> IResult<&str, Vec<usize>> {
    terminated(
        fold_many1(
            terminated(digit1, opt(tag(","))),
            Vec::<usize>::new(),
            |mut v, n: &str| {
                v.push(n.parse().unwrap());
                v
            },
        ),
        multispace0,
    )(input)
}

fn parse_yours<'a>(input: &'a str) -> IResult<&'a str, Vec<usize>> {
    let (input, ()) = value((), tuple((tag("your ticket:"), multispace0)))(input)?;
    ticket(input)
}

fn parse_others<'a>(input: &'a str) -> IResult<&'a str, Vec<Vec<usize>>> {
    let (input, ()) = value((), tuple((tag("nearby tickets:"), multispace0)))(input)?;
    Ok(many1(ticket)(input)?)
}

fn parse_tickets(input: &str) -> IResult<&str, Tickets> {
    let (input, fields) = many1(parse_fields)(input)?;
    let fields = fields.into_iter().collect();
    let (input, yours) = parse_yours(input)?;
    let (input, others) = parse_others(input)?;
    Ok((
        input,
        Tickets {
            fields,
            yours,
            others,
        },
    ))
}

fn test_field(field: &str, val: usize, fields: &MyMap<String, Vec<RangeInclusive<usize>>>) -> bool {
    if let Some(ranges) = fields.get(field) {
        ranges.iter().any(|r| r.contains(&val))
    } else {
        false
    }
}

fn try_fields(
    left: &mut VecDeque<(usize, usize)>,
    guess: &mut MyMap<usize, MySet<String>>,
    fields: &MyMap<String, Vec<RangeInclusive<usize>>>,
) -> Option<MyMap<usize, MySet<String>>> {
    if left.is_empty() {
        return Some(guess.clone());
    }
    let this = left.pop_front().unwrap();
    let possibilities: Vec<String> = fields
        .keys()
        .cloned()
        .filter(|key| test_field(key, this.1, fields))
        .collect();
    if possibilities.is_empty() {
        return None;
    }
    let possibilities = possibilities.iter().cloned().collect();
    let old_possibilities = guess.get(&this.0);
    let possibilities = if let Some(old_possibilities) = old_possibilities {
        old_possibilities
            .intersection(&possibilities)
            .cloned()
            .collect()
    } else {
        possibilities
    };
    guess.insert(this.0, possibilities.clone());
    let fields = if let Ok(possibility) = possibilities.iter().exactly_one() {
        guess.iter_mut().for_each(|(k, v)| {
            if *k != this.0 {
                v.remove(possibility);
            }
        });
        fields
            .iter()
            .filter(|(k, _)| k != &possibility)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    } else {
        fields.clone()
    };
    try_fields(left, guess, &fields)
}

fn get_fields(tickets: &Tickets, good: &Vec<Vec<usize>>) -> MyMap<usize, MySet<String>> {
    let mut map = HashMap::default();
    for other in good.iter() {
        let mut left = other.iter().copied().enumerate().collect();
        let attempt = try_fields(&mut left, &mut map.clone(), &tickets.fields);
        if let Some(attempt) = attempt {
            if attempt.values().all(|v| v.iter().exactly_one().is_ok()) {
                return attempt;
            } else {
                map = attempt;
            }
        } else {
            continue;
        }
        loop {
            let mut removed = 0;

            let singles = map
                .iter()
                .filter(|(_, v)| v.len() == 1)
                .map(|(k, v)| (*k, v.iter().next().unwrap().clone()))
                .collect::<Vec<_>>();
            singles
                .iter()
                .map(|(col, v)| (*col, v.clone()))
                .for_each(|(col, possibility)| {
                    map.iter_mut()
                        .filter(|(&k, _)| k != col)
                        .for_each(|(_, v)| {
                            if v.remove(&possibility) {
                                removed += 1;
                            }
                        })
                });

            if removed == 0 {
                break;
            }
        }
    }

    map
}

impl Runner for Day16 {
    type Input = Tickets;
    type Output = usize;

    fn day() -> usize {
        16
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(parse_tickets(input).unwrap().1)
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .others
            .iter()
            .flatten()
            .filter(|val| {
                !input
                    .fields
                    .values()
                    .flatten()
                    .any(|r| -> bool { r.contains(*val) })
            })
            .sum())
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let good = input
            .others
            .iter()
            .filter(|v| {
                v.iter()
                    .filter(|val| {
                        !input
                            .fields
                            .values()
                            .flatten()
                            .any(|r| -> bool { r.contains(*val) })
                    })
                    .count()
                    == 0
            })
            .cloned()
            .collect::<Vec<Vec<_>>>();
        let fields = get_fields(input, &good);
        Ok(fields
            .iter()
            .filter(|(_, name)| name.iter().next().unwrap().starts_with("departure "))
            .map(|(&i, _): (&usize, _)| input.yours[i])
            .product::<usize>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "class: 1-3 or 5-7\n\
                     row: 6-11 or 33-44\n\
                     seat: 13-40 or 45-50\n\
                     \n\
                     your ticket:\n\
                     7,1,14\n\
                     \n\
                     nearby tickets:\n\
                     7,3,47\n\
                     40,4,50\n\
                     55,2,20\n\
                     38,6,12";

        let input = Day16::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(71, Day16::part1(&input)?);
        // assert_eq!(175594, Day16::part2(&input)?);
        Ok(())
    }

    #[test]
    fn sample2() -> Result<()> {
        let input = "class: 0-1 or 4-19\n\
                     row: 0-5 or 8-19\n\
                     seat: 0-13 or 16-19\n\
                     \n\
                     your ticket:\n\
                     11,12,13\n\
                     \n\
                     nearby tickets:\n\
                     3,9,18\n\
                     15,1,5\n\
                     5,14,9";

        let input = Day16::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(0, Day16::part1(&input)?);
        assert_eq!(175594, Day16::part2(&input)?);
        Ok(())
    }
}
