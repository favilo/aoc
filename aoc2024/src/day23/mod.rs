use std::mem;

use hashbrown::HashSet;
use itertools::Itertools;
use miette::Result;
use petgraph::prelude::GraphMap;
use petgraph::Undirected;

use crate::Runner;

pub struct Day;

fn bron_kerbosch<'input>(
    graph: &GraphMap<&'input str, (), Undirected>,
    r: HashSet<&'input str>,
    mut p: HashSet<&'input str>,
    mut x: HashSet<&'input str>,
) -> Vec<HashSet<&'input str>> {
    let mut cliques = Vec::with_capacity(1);
    if p.is_empty() {
        if x.is_empty() {
            cliques.push(r);
        }
        return cliques;
    }

    let u = p
        .iter()
        .max_by_key(|v| graph.neighbors(v).count())
        .expect("at least one vertex");

    let todo = p
        .iter()
        .filter(|&v| u == v || !graph.contains_edge(u, *v))
        .cloned()
        .collect::<Vec<_>>();

    todo.iter().for_each(|v| {
        let mut neighbors = HashSet::new();
        let walker = graph.neighbors(v);
        walker.for_each(|w| {
            if graph.contains_edge(w, v) {
                neighbors.insert(w);
            }
        });
        p.remove(v);
        let mut next_r = r.clone();
        next_r.insert(v);

        let next_p = p.intersection(&neighbors).cloned().collect::<HashSet<_>>();
        let next_x = x.intersection(&neighbors).cloned().collect::<HashSet<_>>();
        cliques.extend(bron_kerbosch(graph, next_r, next_p, next_x));

        x.insert(v);
    });

    cliques
}

impl Runner<usize, String> for Day {
    type Input<'input> = GraphMap<&'input str, (), Undirected>;

    #[rustfmt::skip]
    fn day() -> usize {
        23
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(input
            .lines()
            .flat_map(|line| {
                let (a, b) = line.split_once('-').unwrap();
                [(a, b), (b, a)]
            })
            .collect())
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let groups = input
            .nodes()
            .filter(|&node| node.starts_with("t"))
            .flat_map(|a| {
                input
                    .neighbors(a)
                    .tuple_combinations()
                    .filter(|(b, c)| input.contains_edge(b, c))
                    .map(move |(b, c)| sorted_tuple((a, b, c)))
            })
            .collect::<HashSet<_>>()
            .len();
        // 83 ms vs 110 µs
        // let groups = input
        //     .nodes()
        //     .tuple_combinations()
        //     .filter(|(a, b, c)| a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
        //     .filter(|(a, b, c)| {
        //         input.contains_edge(a, b) && input.contains_edge(b, c) && input.contains_edge(c, a)
        //     })
        //     .count();

        Ok(groups)
    }

    fn part2(input: &Self::Input<'_>) -> Result<String> {
        let mut cliques = bron_kerbosch(
            input,
            HashSet::new(),
            input.nodes().collect(),
            HashSet::new(),
        );
        cliques.sort_by_key(|clique| -(clique.len() as isize));
        let mut clique: Vec<&str> = Vec::from_iter(cliques.first().cloned().unwrap());
        clique.sort();
        log::debug!("Cliques: {:?}", &clique);
        Ok(clique.join(","))
    }
}

fn sorted_tuple<'input>(
    (mut a, mut b, mut c): (&'input str, &'input str, &'input str),
) -> (&'input str, &'input str, &'input str) {
    if a > b {
        mem::swap(&mut a, &mut b);
    }
    if a > c {
        mem::swap(&mut a, &mut c);
    }
    if b > c {
        mem::swap(&mut b, &mut c);
    }
    (a, b, c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = indoc::indoc! {"
                kh-tc
                qp-kh
                de-cg
                ka-co
                yn-aq
                qp-ub
                cg-tb
                vc-aq
                tb-ka
                wh-tc
                yn-cg
                kh-ub
                ta-co
                de-co
                tc-td
                tb-wq
                wh-td
                ta-ka
                td-qp
                aq-cg
                wq-ub
                ub-vc
                de-ta
                wq-aq
                wq-vc
                wh-yn
                ka-de
                kh-ta
                co-tc
                wh-qp
                tb-vc
                td-yn
            "};
            part1 = 7;
            part2 = "co,de,ka,ta";
    }

    prod_case! {
        part1 = 1218;
        part2 = "ah,ap,ek,fj,fr,jt,ka,ln,me,mp,qa,ql,zg";
    }
}
