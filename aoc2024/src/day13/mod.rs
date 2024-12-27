use miette::Result;
use nalgebra::{Matrix2, Vector2};
use winnow::{
    ascii::{dec_uint, line_ending},
    combinator::{opt, repeat, terminated},
    seq,
    stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial},
    Located, PResult, Parser,
};

use aoc_utils::errors::ToMiette;
use aoc_utils::Runner;

pub struct Day;

#[derive(Debug, Clone)]
pub struct System {
    vars: Matrix2<f64>,
    prize: Vector2<f64>,
}

impl System {
    pub fn solve(&self) -> Option<Vector2<f64>> {
        Some(self.vars.try_inverse()? * self.prize)
    }

    pub fn parser<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr,
    {
        seq!(
            seq!(_: "Button A: ",  Self::button, _: line_ending),
            seq!(_: "Button B: ",  Self::button, _: line_ending),
            seq!(_: "Prize: ",  Self::prize, _: line_ending),
        )
        .map(|((a,), (b,), (prize,))| Self {
            vars: Matrix2::from_columns(&[a, b]),
            prize,
        })
        .parse_next(input)
    }

    pub fn button<S>(input: &mut S) -> PResult<Vector2<f64>>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr,
    {
        seq!(_: "X+", dec_uint::<_, usize, _>, _: ", Y+", dec_uint::<_, usize, _>)
            .map(|(x, y)| Vector2::new(x as f64, y as f64))
            .parse_next(input)
    }

    pub fn prize<S>(input: &mut S) -> PResult<Vector2<f64>>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone,
        <S as Stream>::Slice: AsBStr,
    {
        seq!(_: "X=", dec_uint::<_, usize,_>, _: ", Y=", dec_uint::<_, usize,_>)
            .map(|(x, y)| Vector2::new(x as f64, y as f64))
            .parse_next(input)
    }
}

pub fn systems<S>(input: &mut S) -> PResult<Vec<System>>
where
    for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
    <S as Stream>::Token: AsChar + Clone,
    <S as Stream>::Slice: AsBStr,
{
    repeat(1.., terminated(System::parser, opt(line_ending))).parse_next(input)
}

impl Runner for Day {
    type Input<'input> = Vec<System>;

    fn day() -> usize {
        13
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        systems.parse(Located::new(input)).to_miette()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .iter()
            .flat_map(|sys| sys.solve())
            .filter(|s| s.iter().all(|c| (c - c.round()).abs() < 0.001))
            // .inspect(|s| println!("{s:?}"))
            .map(|s| s.x.round() as usize * 3 + s.y.round() as usize)
            // .inspect(|s| println!("{s:?}"))
            .sum())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .iter()
            .flat_map(|sys| {
                let mut sys = sys.clone();
                sys.prize += Vector2::new(10000000000000.0, 10000000000000.0);
                sys.solve()
            })
            .filter(|s| s.iter().all(|c| (c - c.round()).abs() < 0.001))
            // .inspect(|s| println!("{s:?}"))
            .map(|s| s.x.round() as usize * 3 + s.y.round() as usize)
            // .inspect(|s| println!("{s:?}"))
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                Button A: X+94, Y+34\n\
                Button B: X+22, Y+67\n\
                Prize: X=8400, Y=5400\n\
                \n\
                Button A: X+26, Y+66\n\
                Button B: X+67, Y+21\n\
                Prize: X=12748, Y=12176\n\
                \n\
                Button A: X+17, Y+86\n\
                Button B: X+84, Y+37\n\
                Prize: X=7870, Y=6450\n\
                \n\
                Button A: X+69, Y+23\n\
                Button B: X+27, Y+71\n\
                Prize: X=18641, Y=10279\n\
            ";
            part1 = 480;
            part2 = 875318608908_usize;
    }

    prod_case! {
        part1 = 36250;
        part2 = 83232379451012;
    }
}
