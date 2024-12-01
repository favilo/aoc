use color_eyre::Result;
use itertools::Itertools;
use ndarray::Array2;
use petgraph::{algo::dijkstra, graph::NodeIndex, Graph};

use crate::{utils::four_neighbors, Runner};

pub struct Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Height {
    Start,
    End,
    Height(usize),
}

impl Height {
    fn from_byte(b: u8) -> Self {
        match b {
            b'S' => Self::Start,
            b'E' => Self::End,
            c @ b'a'..=b'z' => Self::Height(c as _),
            _ => unreachable!(),
        }
    }

    fn height(self) -> usize {
        match self {
            Height::Start => b'a' as _,
            Height::End => b'z' as _,
            Height::Height(height) => height,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    _coords: (usize, usize),
    _height: Height,
}

impl Runner for Day {
    type Input<'input> = (
        Graph<(), ()>,
        NodeIndex<u32>,
        NodeIndex<u32>,
        Vec<NodeIndex<u32>>,
    );

    fn day() -> usize {
        12
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let mut start = None;
        let mut other_starts = vec![];
        let mut end = None;
        let v = input
            .lines()
            .flat_map(|line| line.bytes())
            .map(|b| (Height::from_byte(b), None))
            .collect_vec();
        let mut graph = Graph::with_capacity(height * width, height * width);
        let mut array = Array2::from_shape_vec((height, width), v)?;
        // Add nodes to graph
        array.indexed_iter_mut().for_each(|((r, c), h)| {
            let _p = Position {
                _coords: (r, c),
                _height: h.0,
            };
            h.1 = Some(graph.add_node(()));
        });

        // Add edges to graph
        array.indexed_iter().for_each(|((r, c), (h, id))| {
            if *h == Height::Start {
                start = *id;
            }
            if h.height() == Height::Start.height() {
                other_starts.push(id.unwrap());
            }
            if *h == Height::End {
                end = *id;
            }
            four_neighbors((r, c), (height, width))
                .filter_map(|(or, oc)| {
                    let other = array[(or, oc)];
                    let height = other.0.height();
                    let diff = height as isize - h.height() as isize;
                    (diff < 2).then_some(other.1)
                })
                .for_each(|o_id| {
                    graph.update_edge(id.unwrap(), o_id.unwrap(), ());
                });
        });

        Ok((graph, start.unwrap(), end.unwrap(), other_starts))
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let (graph, start, end, _) = input;

        let path = dijkstra(graph, *start, None, |_| 1);
        Ok(*path.get(end).unwrap())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let (graph, _, end, other_starts) = input;
        let mut graph = graph.clone();
        graph.reverse();

        let paths = dijkstra(&graph, *end, None, |_| 1);
        Ok(other_starts
            .iter()
            .map(|id| *paths.get(id).unwrap_or(&i32::MAX))
            .min()
            .unwrap() as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                Sabqponm\n\
                abcryxxl\n\
                accszExk\n\
                acctuvwj\n\
                abdefghi\n";
            part1 = 31;
            part2 = 29;
    }

    prod_case! {
        part1 = 361;
        part2 = 354;
    }
}
