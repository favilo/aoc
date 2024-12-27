use std::{
    borrow::Cow,
    cmp::{max, min},
};

use hashbrown::HashMap;
use itertools::Itertools;
use layout::{
    backends::svg::SVGWriter,
    core::{base::Orientation, geometry::Point, style::StyleAttr, utils::save_to_file},
    std_shapes::shapes::{Arrow, Element, ShapeKind},
    topo::layout::VisualGraph,
};
use miette::Result;
use winnow::{
    ascii::{alphanumeric1, dec_uint, line_ending, space1},
    combinator::{alt, delimited, repeat},
    seq, PResult, Parser,
};

use crate::errors::ToMiette;
use aoc_utils::Runner;

pub struct Day;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Gate<'input> {
    And(Cow<'input, str>, Cow<'input, str>),
    Or(Cow<'input, str>, Cow<'input, str>),
    Xor(Cow<'input, str>, Cow<'input, str>),
}

impl<'input> Gate<'input> {
    fn name(&self) -> &'static str {
        match self {
            Gate::And(_, _) => "AND",
            Gate::Or(_, _) => "OR",
            Gate::Xor(_, _) => "XOR",
        }
    }

    fn first(&'input self) -> &'input str {
        match self {
            Gate::And(a, _) => a.as_ref(),
            Gate::Or(a, _) => a.as_ref(),
            Gate::Xor(a, _) => a.as_ref(),
        }
    }

    fn second(&self) -> &str {
        match self {
            Gate::And(_, b) => b.as_ref(),
            Gate::Or(_, b) => b.as_ref(),
            Gate::Xor(_, b) => b.as_ref(),
        }
    }
}

fn input_line<'input>(input: &mut &'input str) -> PResult<(Cow<'input, str>, bool)> {
    seq! { alphanumeric1.map(Cow::from), _:": ", dec_uint.map(|b: usize| b != 0), _: line_ending }
        .parse_next(input)
}

fn inputs<'input>(input: &mut &'input str) -> PResult<HashMap<Cow<'input, str>, bool>> {
    repeat(1.., input_line)
        .map(|v: Vec<(_, bool)>| HashMap::from_iter(v))
        .parse_next(input)
}

fn gate<'input>(input: &mut &'input str) -> PResult<&'input str> {
    delimited(space1, alt(("AND", "OR", "XOR")), space1).parse_next(input)
}

fn edge<'input>(input: &mut &'input str) -> PResult<(Gate<'input>, Cow<'input, str>)> {
    seq! { alphanumeric1.map(Cow::from), gate, alphanumeric1.map(Cow::from), _:" -> ", alphanumeric1.map(Cow::from), _: line_ending }
        .map(|(a, g, b, out)| match g {
            "AND" => (Gate::And(min(a.clone(), b.clone()), max(a, b)), out),
            "OR" => (Gate::Or(min(a.clone(), b.clone()), max(a, b)), out),
            "XOR" => (Gate::Xor(min(a.clone(), b.clone()), max(a, b)), out),
            _ => unreachable!(),
        })
        .parse_next(input)
}

fn edges<'input>(input: &mut &'input str) -> PResult<Vec<(Gate<'input>, Cow<'input, str>)>> {
    repeat(1.., edge).parse_next(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Circuit<'input> {
    inputs: HashMap<Cow<'input, str>, bool>,
    edges: HashMap<Cow<'input, str>, Gate<'input>>,
    outputs: Vec<Cow<'input, str>>,

    collapsed: HashMap<Cow<'input, str>, bool>,
}

impl<'input> Circuit<'input> {
    fn parser(input: &mut &'input str) -> PResult<Self> {
        seq! { inputs, _: line_ending, edges }
            .map(|(inputs, edges)| {
                let mut outputs = edges
                    .iter()
                    .filter_map(|(_g, o)| o.starts_with("z").then_some(o.clone()))
                    .collect_vec();
                outputs.sort();
                Self {
                    collapsed: inputs.clone(),

                    inputs,
                    edges: edges.into_iter().map(|(g, o)| (o, g)).collect(),
                    outputs,
                }
            })
            .parse_next(input)
    }

    fn collapse(&mut self, wire: Cow<'input, str>) -> bool {
        if let Some(value) = self.collapsed.get(&wire) {
            return *value;
        }

        let Some(gate) = self.edges.get(&wire).cloned() else {
            panic!("no gate for {wire}");
        };
        // log::debug!("Collapsing `{wire}` -> Gate: {gate:?}");
        let value = match gate {
            Gate::And(a, b) => self.collapse(a) && self.collapse(b),
            Gate::Or(a, b) => self.collapse(a) || self.collapse(b),
            Gate::Xor(a, b) => self.collapse(a) ^ self.collapse(b),
        };
        // log::debug!("Value: {value}");

        self.collapsed.insert(wire, value);
        value
    }

    fn run(&mut self) -> Result<usize> {
        let outputs = std::mem::take(&mut self.outputs);
        let bools_to_usize = bools_to_usize(outputs.iter().rev().map(|o| self.collapse(o.clone())));
        self.outputs = outputs;
        Ok(bools_to_usize)
    }

    #[allow(dead_code)]
    fn debug_print(&self) {
        let mut vg = VisualGraph::new(Orientation::LeftToRight);

        let mut nodes = HashMap::new();
        nodes.extend(self.inputs.keys().map(|k| {
            let element = Element::create(
                ShapeKind::new_double_circle(k.as_ref()),
                StyleAttr::simple(),
                Orientation::TopToBottom,
                Point::new(100., 100.),
            );
            (k.as_ref(), vg.add_node(element))
        }));
        nodes.extend(self.edges.keys().map(|k| {
            let element = Element::create(
                ShapeKind::new_double_circle(k.as_ref()),
                StyleAttr::simple(),
                Orientation::TopToBottom,
                Point::new(100., 100.),
            );
            (k.as_ref(), vg.add_node(element))
        }));

        for (k, v) in &self.edges {
            let gate = Element::create(
                ShapeKind::new_box(v.name()),
                StyleAttr::simple(),
                Orientation::TopToBottom,
                Point::new(100., 100.),
            );
            let gate = vg.add_node(gate);
            let arrow = Arrow::simple("");
            vg.add_edge(arrow.clone(), nodes[v.first()], gate);
            vg.add_edge(arrow.clone(), nodes[v.second()], gate);
            vg.add_edge(arrow.clone(), gate, nodes[k.as_ref()]);
        }

        let mut svg = SVGWriter::new();
        vg.do_it(false, false, false, &mut svg);

        save_to_file("./day24-graph.svg", &svg.finalize()).unwrap();
    }

    fn ripple_adder_structure(&self) -> Vec<Cow<'input, str>> {
        let mut bad = self
            .edges
            .iter()
            .filter_map(|(wire, gate)| {
                if wire.starts_with("z") && wire != "z45" {
                    // If we are an output, we are an XOR, unless it's z45
                    match gate {
                        Gate::And(_, _) | Gate::Or(_, _) => {
                            log::debug!("Bad output gate: {wire}, {gate:?}");
                            Some(wire.clone())
                        }
                        _ => None,
                    }
                } else if !(gate.first().starts_with("x")
                    || gate.second().starts_with("y")
                    || gate.second().starts_with("x")
                    || gate.first().starts_with("y"))
                {
                    // If we aren't a direct input, we can't be an XOR
                    match gate {
                        Gate::Xor(_, _) => {
                            log::debug!("Bad XOR gate: {wire}, {gate:?}");
                            Some(wire.clone())
                        }
                        _ => None,
                    }
                }
                // If `gate` is x{nn} XOR y{nn}:
                // - `wire` must be XOR'd with something later
                // If `gate` is x{nn} AND y{nn}:
                // - `wire` must be OR'd with something later
                else if gate.first().starts_with("x") && gate.second().starts_with("y")
                    || gate.second().starts_with("x") && gate.first().starts_with("y")
                {
                    // x00 and y00 are special cases that don't need to be matched here.
                    if gate.first().ends_with("00") && gate.second().ends_with("00") {
                        return None;
                    }

                    let matching_gates = self
                        .edges
                        .iter()
                        .filter(|(_, g)| g.first() == wire || g.second() == wire)
                        .collect::<Vec<_>>();
                    // log::debug!("Gates matching this wire `{wire}`, {gate:?}: {matching_gates:?}");
                    if matches!(gate, Gate::Xor(_, _))
                        && !matching_gates
                            .iter()
                            .any(|(_, g)| matches!(g, Gate::Xor(_, _)))
                    {
                        log::debug!(
                            "XOR without matching XOR `{wire}`, {gate:?}: {matching_gates:?}"
                        );
                        Some(wire.clone())
                    } else if matches!(gate, Gate::And(_, _))
                        && !matching_gates
                            .iter()
                            .any(|(_, g)| matches!(g, Gate::Or(_, _)))
                    {
                        log::debug!(
                            "AND without matching OR `{wire}`, {gate:?}: {matching_gates:?}"
                        );
                        Some(wire.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        bad.sort();
        bad
    }
}

fn bools_to_usize(slice: impl Iterator<Item = bool>) -> usize {
    slice.fold(0, |a, c| a * 2 + c as usize)
}

impl Runner<usize, String> for Day {
    type Input<'input> = Circuit<'input>;

    #[rustfmt::skip]
    fn day() -> usize {
        24
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Circuit::parser.parse(input).to_miette()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut circuit = input.clone();
        circuit.run()
    }

    fn part2(input: &Self::Input<'_>) -> Result<String> {
        let circuit = input.clone();
        // circuit.debug_print();
        let bad = circuit.ripple_adder_structure();
        log::debug!("{bad:?}");
        Ok(bad.iter().join(","))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input1 = indoc::indoc! {"
                x00: 1
                x01: 1
                x02: 1
                y00: 0
                y01: 1
                y02: 0

                x00 AND y00 -> z00
                x01 XOR y01 -> z01
                x02 OR y02 -> z02
            "};
            part1 = 4;
            input2 = indoc::indoc! {"
                x00: 0
                x01: 1
                x02: 0
                x03: 1
                x04: 0
                x05: 1
                y00: 0
                y01: 0
                y02: 1
                y03: 1
                y04: 0
                y05: 1

                x00 AND y00 -> z05
                x01 AND y01 -> z02
                x02 AND y02 -> z01
                x03 AND y03 -> z03
                x04 AND y04 -> z04
                x05 AND y05 -> z00
            "};
            // This is not the correct answer for this input, but I'm matching the rules for the
            // ripple carry adder structure.
            part2 = "z00,z01,z02,z03,z04,z05";
    }

    sample_case! {
        sample2 =>
            input = indoc::indoc! {"
                x00: 1
                x01: 0
                x02: 1
                x03: 1
                x04: 0
                y00: 1
                y01: 1
                y02: 1
                y03: 1
                y04: 1

                ntg XOR fgs -> mjb
                y02 OR x01 -> tnw
                kwq OR kpj -> z05
                x00 OR x03 -> fst
                tgd XOR rvg -> z01
                vdt OR tnw -> bfw
                bfw AND frj -> z10
                ffh OR nrd -> bqk
                y00 AND y03 -> djm
                y03 OR y00 -> psh
                bqk OR frj -> z08
                tnw OR fst -> frj
                gnj AND tgd -> z11
                bfw XOR mjb -> z00
                x03 OR x00 -> vdt
                gnj AND wpb -> z02
                x04 AND y00 -> kjc
                djm OR pbm -> qhw
                nrd AND vdt -> hwm
                kjc AND fst -> rvg
                y04 OR y02 -> fgs
                y01 AND x02 -> pbm
                ntg OR kjc -> kwq
                psh XOR fgs -> tgd
                qhw XOR tgd -> z09
                pbm OR djm -> kpj
                x03 XOR y03 -> ffh
                x00 XOR y04 -> ntg
                bfw OR bqk -> z06
                nrd XOR fgs -> wpb
                frj XOR qhw -> z04
                bqk OR frj -> z07
                y03 OR x01 -> nrd
                hwm AND bqk -> z03
                tgd XOR rvg -> z12
                tnw OR pbm -> gnj
            "};
            part1 = 2024;
            // This is not the correct answer for this input, but I'm matching the rules for the
            // ripple carry adder structure.
            part2 = "ffh,mjb,tgd,wpb,z02,z03,z05,z06,z07,z08,z10,z11";
    }

    prod_case! {
        part1 = 65740327379952;
        part2 = "bgs,pqc,rjm,swt,wsv,z07,z13,z31";
    }
}
