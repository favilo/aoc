use std::{cmp::Ordering, fmt::Debug, hash::Hash, str::FromStr};

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline},
    combinator::map,
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};
use petgraph::{graphmap::UnGraphMap, IntoWeightedEdge};

use crate::Runner;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Node {
    Start,
    End,
    Large(u16),
    Small(u16),
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "start"),
            Self::End => write!(f, "end"),
            Self::Large(arg0) => write!(f, "{}", from_u16(*arg0)),
            Self::Small(arg0) => write!(f, "{}", from_u16(*arg0)),
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Start, Self::Start) => Ordering::Equal,
            (Self::End, Self::End) => Ordering::Equal,
            (Self::Start, _) => Ordering::Less,
            (Self::End, _) => Ordering::Greater,
            (_, Self::Start) => Ordering::Greater,
            (_, Self::End) => Ordering::Less,
            (Self::Large(s), Self::Large(o)) => s.cmp(&o),
            (Self::Large(_), Self::Small(_)) => Ordering::Greater,
            (Self::Small(_), Self::Large(_)) => Ordering::Less,
            (Self::Small(s), Self::Small(o)) => s.cmp(&o),
        }
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "start" => Self::Start,
            "end" => Self::End,
            node => {
                let s = to_u16(s);
                if node.chars().next().unwrap().is_lowercase() {
                    Self::Small(s)
                } else {
                    Self::Large(s)
                }
            }
        })
    }
}

fn to_u16(s: &str) -> u16 {
    let s = &s.as_bytes();
    if s.len() == 2 {
        u16::from_be_bytes([s[0], s[1]])
    } else {
        u16::from_be_bytes([0, s[0]])
    }
}

fn from_u16(i: u16) -> String {
    if i >= 256 {
        (i as u16)
            .to_be_bytes()
            .iter()
            .map(|&b| b as u8 as char)
            .collect()
    } else {
        (i as u8 as char).to_string()
    }
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Edge(Node, Node);

impl IntoWeightedEdge<()> for Edge {
    type NodeId = u64;

    fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, ()) {
        (self.0.node_weight(), self.1.node_weight(), ())
    }
}
impl Node {
    fn node_weight(&self) -> u64 {
        match self {
            Node::Start => 0,
            Node::End => u64::MAX,
            Node::Large(u) => *u as u64,
            Node::Small(u) => *u as u64,
        }
    }
}

pub struct Day;

impl Runner for Day {
    type Input = UnGraphMap<Node, ()>;

    fn day() -> usize {
        12
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        let (input, edges) = many1(edge)(input).map_err(|_| anyhow::anyhow!("Error"))?;
        assert_eq!("", input);

        Ok(UnGraphMap::<Node, ()>::from_edges(&edges))
    }

    fn part1(graph: &Self::Input) -> Result<usize> {
        Ok(count_paths(
            graph,
            &mut vec![Node::Start],
            &mut vec![],
            true,
        ))
    }

    fn part2(graph: &Self::Input) -> Result<usize> {
        Ok(count_paths(
            graph,
            &mut vec![Node::Start],
            &mut vec![],
            false,
        ))
    }
}

fn count_paths(
    graph: &<Day as Runner>::Input,
    path: &mut Vec<Node>,
    visited: &mut Vec<Node>,
    twice_done: bool,
) -> usize {
    let start = path[path.len() - 1];
    if start == Node::End {
        return 1;
    }
    visited.push(start);
    let sum = graph
        .neighbors(start)
        .map(|n| {
            if !visited.contains(&n) || matches!(n, Node::Large(_)) {
                path.push(n);
                let c = count_paths(graph, path, visited, twice_done);
                path.pop();
                c
            } else if visited.contains(&n) && matches!(n, Node::Small(_)) && !twice_done {
                path.push(n);
                let c = count_paths(graph, path, visited, true);
                path.pop();
                c
            } else {
                0
            }
        })
        .sum();
    visited.pop();
    sum
}

fn edge<'a>(input: &'a str) -> IResult<&'a str, (Node, Node)> {
    Ok(tuple((
        map(terminated(alphanumeric1, tag("-")), |n| {
            Node::from_str(n).unwrap()
        }),
        map(terminated(alphanumeric1, newline), |n| {
            Node::from_str(n).unwrap()
        }),
    ))(input)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(10, Day::part1(&input)?);
        assert_eq!(36, Day::part2(&input)?);
        Ok(())
    }
}
