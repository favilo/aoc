use std::{cmp::Reverse, collections::HashMap};

use color_eyre::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u64},
    combinator::map,
    error::VerboseError,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use petgraph::{algo::floyd_warshall, graph::NodeIndex, Directed, Graph};

use crate::Runner;

pub struct Day;

type SInput<'a> = &'a str;
type ParseResult<'a, T> = IResult<SInput<'a>, T, VerboseError<SInput<'a>>>;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Set(u64);

impl Set {
    fn insert(self, position: usize) -> Self {
        assert!(position < 64);
        Set(self.0 | (1 << position))
    }

    fn contains(self, position: usize) -> bool {
        assert!(position < 64);
        self.0 & (1 << position) != 0
    }

    fn union(self, other: Self) -> Self {
        Set(self.0 | other.0)
    }

    fn intersection(self, other: Self) -> Self {
        Set(self.0 & other.0)
    }

    fn count(self) -> u32 {
        self.0.count_ones()
    }
}

#[derive(Debug, Clone)]
pub struct Valve {
    name: String,
    rate: u64,
    connections: Vec<String>,
}

fn valve(input: SInput) -> ParseResult<Valve> {
    map(
        tuple((
            preceded(tag("Valve "), map(alpha1, str::to_string)),
            preceded(tag(" has flow rate="), u64),
            preceded(
                alt((
                    tag("; tunnels lead to valves "),
                    tag("; tunnel leads to valve "),
                )),
                separated_list1(tag(", "), map(alpha1, str::to_string)),
            ),
        )),
        |(name, rate, connections)| Valve {
            name,
            rate,
            connections,
        },
    )(input)
}

impl Runner for Day {
    type Input<'input> = (HashMap<String, NodeIndex>, Graph<Valve, u64, Directed>);

    fn day() -> usize {
        16
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        let mut graph = Graph::new();
        let mut names = HashMap::new();
        input
            .lines()
            .map(|line| valve(line).unwrap().1)
            .for_each(|valve| {
                names.insert(valve.name.clone(), graph.add_node(valve));
            });
        names.iter().for_each(|(_, id)| {
            let valve = graph.node_weight(*id).unwrap().clone();
            valve.connections.iter().for_each(|name| {
                let other_id = names.get(name).unwrap();
                graph.update_edge(*id, *other_id, 1);
                graph.update_edge(*other_id, *id, 1);
            });
        });

        Ok((names, graph))
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let (names, graph) = input;
        let start = *names.get("AA").unwrap();
        let (shortest_paths, sorted_by_flow_rates) = process_input(graph);

        let mut pressure = 0;
        branch_and_bound(
            graph,
            &sorted_by_flow_rates,
            &shortest_paths,
            State::new(start, 30),
            &mut None,
            &mut pressure,
            |bound, best| bound > best,
        );
        Ok(pressure as usize)
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let (names, graph) = input;
        let start = *names.get("AA").unwrap();
        let (shortest_paths, sorted_by_flow_rates) = process_input(graph);
        let mut best_per_visited = HashMap::new();
        branch_and_bound(
            graph,
            &sorted_by_flow_rates,
            &shortest_paths,
            State::new(start, 26),
            &mut Some(&mut best_per_visited),
            &mut 0,
            |bound, best| bound > best * 3 / 4,
        );
        let best_per_visited = best_per_visited
            .into_iter()
            .filter(|&(_, best)| best > 0)
            .sorted_unstable_by_key(|&(_, best)| Reverse(best))
            .collect_vec();
        let mut best = 0;
        for (i, &(person_visited, person_best)) in best_per_visited.iter().enumerate() {
            for &(elephant_visited, elephant_best) in &best_per_visited[i + 1..] {
                let score = person_best + elephant_best;
                if score <= best {
                    break;
                }
                if person_visited.intersection(elephant_visited).count() == 0 {
                    best = score;
                    break;
                }
            }
        }
        Ok(best as usize)
    }
}

fn process_input(
    graph: &Graph<Valve, u64>,
) -> (HashMap<NodeIndex, HashMap<NodeIndex, u64>>, Vec<NodeIndex>) {
    let shortest_all_paths = floyd_warshall(graph, |e| *e.weight()).unwrap();
    let interesting_valve_indices = graph
        .node_indices()
        .filter(|id| {
            let valve = graph.node_weight(*id).unwrap();
            valve.name == "AA" || valve.rate > 0
        })
        .collect_vec();
    let shortest_paths = interesting_valve_indices
        .iter()
        .map(|&i| {
            (
                i,
                interesting_valve_indices
                    .iter()
                    .map(|&j| (j, shortest_all_paths[&(i, j)]))
                    .collect(),
            )
        })
        .collect();
    let sorted_by_flow_rates: Vec<NodeIndex> = interesting_valve_indices
        .iter()
        .map(|&i| (i, graph.node_weight(i).unwrap()))
        .sorted_unstable_by_key(|v| Reverse(v.1.rate))
        .map(|(i, _)| i)
        .collect_vec();
    (shortest_paths, sorted_by_flow_rates)
}

fn branch_and_bound(
    graph: &Graph<Valve, u64, Directed>,
    sorted_flows: &[NodeIndex],
    shortest_paths: &HashMap<NodeIndex, HashMap<NodeIndex, u64>>,
    state: State,
    best_for_visited: &mut Option<&mut HashMap<Set, u64>>,
    best: &mut u64,
    filter_bound: impl Fn(u64, u64) -> bool + Copy,
) {
    best_for_visited.as_mut().map(|best_for_visited| {
        *best_for_visited
            .entry(state.visited)
            .and_modify(move |cur_best| *cur_best = state.pressure_released.max(*cur_best))
            .or_insert(state.pressure_released)
    });

    *best = state.pressure_released.max(*best);
    let bound_and_branch_pairs = state
        .branch(graph, shortest_paths)
        .into_iter()
        .map(|state: State| (state.bound(graph, sorted_flows), state))
        .filter(|&(bound, _)| filter_bound(bound, *best))
        .sorted_unstable_by_key(|(bound, _)| Reverse(*bound))
        .collect_vec();
    for (bound, branch) in bound_and_branch_pairs {
        if filter_bound(bound, *best) {
            branch_and_bound(
                graph,
                sorted_flows,
                shortest_paths,
                branch,
                best_for_visited,
                best,
                filter_bound,
            );
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct State {
    visited: Set,
    avoid: Set,
    pressure_released: u64,
    minutes_remaining: u64,
    position: NodeIndex,
}

impl State {
    fn new(position: NodeIndex, minutes_remaining: u64) -> Self {
        Self {
            visited: Default::default(),
            avoid: Set(1 << position.index()),
            pressure_released: 0,
            minutes_remaining,
            position,
        }
    }

    fn can_visit(self, n: NodeIndex) -> bool {
        !self.visited.union(self.avoid).contains(n.index())
    }

    fn bound(self, graph: &Graph<Valve, u64, Directed>, sorted_flows: &[NodeIndex]) -> u64 {
        self.pressure_released
            + (0_u64..=self.minutes_remaining)
                .rev()
                .step_by(2)
                .skip(1)
                .zip(
                    sorted_flows
                        .iter()
                        .filter(|&&i| self.can_visit(i))
                        .map(|&i| graph.node_weight(i).unwrap().rate),
                )
                .map(|(minutes, rate)| minutes * rate)
                .sum::<u64>()
    }

    fn branch<'a>(
        self,
        graph: &'a Graph<Valve, u64, Directed>,
        shortest_paths: &'a HashMap<NodeIndex, HashMap<NodeIndex, u64>>,
    ) -> impl IntoIterator<Item = Self> + 'a {
        shortest_paths[&self.position]
            .iter()
            .filter(move |&(dest, _)| self.can_visit(*dest))
            .filter_map(move |(dest, distance)| {
                let rate = graph.node_weight(*dest).unwrap().rate;
                let minutes_remaining = self.minutes_remaining.checked_sub(*distance + 1)?;
                Some(State {
                    visited: self.visited.insert(dest.index()),
                    avoid: self.avoid,
                    pressure_released: self.pressure_released + minutes_remaining * rate,
                    minutes_remaining,
                    position: *dest,
                })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n\
                Valve BB has flow rate=13; tunnels lead to valves CC, AA\n\
                Valve CC has flow rate=2; tunnels lead to valves DD, BB\n\
                Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n\
                Valve EE has flow rate=3; tunnels lead to valves FF, DD\n\
                Valve FF has flow rate=0; tunnels lead to valves EE, GG\n\
                Valve GG has flow rate=0; tunnels lead to valves FF, HH\n\
                Valve HH has flow rate=22; tunnel leads to valve GG\n\
                Valve II has flow rate=0; tunnels lead to valves AA, JJ\n\
                Valve JJ has flow rate=21; tunnel leads to valve II\n";
            part1 = 1651;
            part2 = 1707;
    }

    prod_case! {
        part1 = 2119;
        part2 = 2615;
    }
}
