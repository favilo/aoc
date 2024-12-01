use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;
use hashers::fx_hash::fxhash;
use multimap::MultiMap;
use nom::character::complete::space0;
use nom::combinator::value;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    combinator::opt,
    multi::many1,
    sequence::tuple,
    IResult,
};
use petgraph::{graphmap::DiGraphMap, Direction};

use crate::Runner;

lazy_static::lazy_static! {
    static ref SHINY_GOLD: u64 = fxhash(b"shiny gold");
}

#[allow(dead_code)]
fn parse_clause<'a>(input: &'a str) -> IResult<&'a str, (usize, u64)> {
    let (input, (num, (), bag, (), _bag, _s, _comma)) = tuple((
        digit1,
        value((), space0),
        take_until(" bag"),
        value((), space0),
        tag("bag"),
        opt(tag("s")),
        opt(tag(", ")),
    ))(input)?;
    let bag_hash = fxhash(bag.as_bytes());
    // print!("({}, '{}'={}), ", num, bag, bag_hash);
    Ok((input, (num.parse().unwrap(), bag_hash)))
}

type BagDef = (u64, Vec<(usize, u64)>);

#[allow(dead_code)]
fn parse_bag<'a>(input: &'a str) -> IResult<&'a str, BagDef> {
    let (input, name) = take_until(" bags contain ")(input)?;
    let (input, _) = tag(" bags contain ")(input)?;
    let (input, opt_none) = opt(tag("no other bags"))(input)?;
    let name_hash = fxhash(name.as_bytes());
    // print!("'{}'={} -> ", name, name_hash);
    let (input, v) = if opt_none.is_some() {
        // print!("[]");
        (input, Vec::new())
    } else {
        many1(parse_clause)(input)?
    };
    let (input, _dot) = tag(".")(input)?;
    assert_eq!("", input);
    // println!();
    Ok((input, (name_hash, v)))
}

pub struct Day07;
pub struct Day07Dag;

impl Runner for Day07Dag {
    type Input = DiGraphMap<u64, usize>;

    type Output = usize;

    fn day() -> usize {
        7
    }

    fn comment() -> &'static str {
        "DAG"
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        // let mut dag = Dag::<String, usize>::new();
        let bags = input
            .lines()
            .map(parse_bag)
            .map(Result::unwrap)
            .map(|(_, (b, vec))| vec.into_iter().map(move |(c, o)| (b, o, c)))
            .flatten();
        let dag: DiGraphMap<_, usize> = DiGraphMap::from_edges(bags);
        Ok(dag)
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(*SHINY_GOLD);
        while !queue.is_empty() {
            let n = queue.pop_front().unwrap();
            if visited.contains(&n) {
                continue;
            }
            if n != *SHINY_GOLD {
                visited.insert(n);
            }
            input
                .neighbors_directed(n, Direction::Incoming)
                .for_each(|n| {
                    queue.push_back(n);
                });
        }

        Ok(visited.len())
    }

    fn part2(_input: &Self::Input) -> Result<Self::Output> {
        todo!()
    }
}

// TODO: Use `nom`, instead of regex

impl Runner for Day07 {
    type Input = HashMap<u64, Vec<(usize, u64)>>;
    type Output = usize;

    fn day() -> usize {
        7
    }

    fn comment() -> &'static str {
        "Regex"
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .lines()
            .map(parse_bag)
            .map(Result::unwrap)
            .map(|(_, b)| b)
            .collect::<HashMap<_, _>>())
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        let contained_in = input
            .iter()
            .map(|(bag, contains)| contains.iter().map(move |(_, contain)| (*contain, *bag)))
            .flatten()
            .collect::<MultiMap<u64, u64>>();
        let mut count = 0;
        let mut queue: VecDeque<u64> = VecDeque::new();
        let mut visited: HashSet<u64> = HashSet::new();
        visited.insert(*SHINY_GOLD);
        queue.push_back(*SHINY_GOLD);
        while !queue.is_empty() {
            let bag = queue.pop_front().unwrap();
            if !visited.contains(&bag) {
                count += 1;
            }
            visited.insert(bag);
            let contained_in = contained_in.get_vec(&bag);
            for o in contained_in.unwrap_or(&vec![]) {
                queue.push_back(*o);
            }
        }
        Ok(count)
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        let mut queue = VecDeque::<(usize, u64)>::new();
        queue.push_back((1, *SHINY_GOLD));
        let mut count: usize = 0;
        let empty = Vec::new();
        while !queue.is_empty() {
            let (i, bag) = queue.pop_front().unwrap();
            count += i as usize;
            let inside = input.get(&bag).unwrap_or(&empty);
            for (o, o_bag) in inside {
                queue.push_back((i * o, *o_bag));
            }
        }
        Ok(count - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn sample1() -> Result<()> {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
                     dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
                     bright white bags contain 1 shiny gold bag.\n\
                     muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
                     shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
                     dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
                     vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
                     faded blue bags contain no other bags.\n\
                     dotted black bags contain no other bags.";

        let input = Day07::get_input(input)?;
        println!("{:#?}", input);
        assert_eq!(4, Day07::part1(&input)?);
        assert_eq!(32, Day07::part2(&input)?);
        Ok(())
    }

    #[test]
    #[ignore]
    fn sample2() -> Result<()> {
        let input = "shiny gold bags contain 2 dark red bags.\n\
                     dark red bags contain 2 dark orange bags.\n\
                     dark orange bags contain 2 dark yellow bags.\n\
                     dark yellow bags contain 2 dark green bags.\n\
                     dark green bags contain 2 dark blue bags.\n\
                     dark blue bags contain 2 dark violet bags.\n\
                     dark violet bags contain no other bags.";

        let input = Day07::get_input(input)?;
        println!("{:#?}", input);
        // assert_eq!(4, Day07::part1(&input)?);
        assert_eq!(126, Day07::part2(&input)?);
        Ok(())
    }

    #[test]
    fn sample1_dag() -> Result<()> {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
                     dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
                     bright white bags contain 1 shiny gold bag.\n\
                     muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
                     shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
                     dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
                     vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
                     faded blue bags contain no other bags.\n\
                     dotted black bags contain no other bags.";

        let input = Day07Dag::get_input(input)?;
        // println!("{:#?}", input);
        assert_eq!(4, Day07Dag::part1(&input)?);
        assert_eq!(32, Day07Dag::part2(&input)?);
        Ok(())
    }

    #[test]
    fn sample2_dag() -> Result<()> {
        let input = "shiny gold bags contain 2 dark red bags.\n\
                     dark red bags contain 2 dark orange bags.\n\
                     dark orange bags contain 2 dark yellow bags.\n\
                     dark yellow bags contain 2 dark green bags.\n\
                     dark green bags contain 2 dark blue bags.\n\
                     dark blue bags contain 2 dark violet bags.\n\
                     dark violet bags contain no other bags.";

        let input = Day07Dag::get_input(input)?;
        println!("{:#?}", input);
        // assert_eq!(4, Day07Dag::part1(&input)?);
        // assert_eq!(126, Day07Dag::part2(&input)?);
        Ok(())
    }
}
