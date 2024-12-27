use std::sync::atomic::{AtomicUsize, Ordering};

use aoc_utils::{graph::four_neighbors, math::coord::Coord};
use hashbrown::HashSet;
use miette::Result;
use winnow::{
    ascii::{dec_int, line_ending},
    combinator::{repeat, separated_pair, terminated},
    stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial},
    PResult, Parser,
};

use aoc_utils::errors::ToMiette;
use aoc_utils::Runner;

static LIMIT: AtomicUsize = AtomicUsize::new(70);
static COUNT: AtomicUsize = AtomicUsize::new(1024);

pub struct Day;

fn coord<S>(input: &mut S) -> PResult<Coord>
where
    for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
    <S as Stream>::Token: AsChar + Clone,
    <S as Stream>::Slice: AsBStr + AsRef<str>,
{
    terminated(separated_pair(dec_int, ",", dec_int), line_ending)
        .map(|(x, y)| Coord(x, y))
        .parse_next(input)
}

impl Runner<usize, String> for Day {
    type Input<'input> = Vec<Coord>;

    fn day() -> usize {
        18
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        repeat(1.., coord).parse(input).to_miette()
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let memory =
            HashSet::<Coord>::from_iter(input.iter().copied().take(COUNT.load(Ordering::Relaxed)));
        let limit = LIMIT.load(Ordering::Relaxed) as isize;

        let path = pathfinding::directed::astar::astar(
            &Coord(0, 0),
            |&c| {
                four_neighbors(c, (limit + 1, limit + 1))
                    .filter(|c| !memory.contains(c))
                    .map(|c| (c, 1))
            },
            |_| 1,
            |&c| c == Coord(limit, limit),
        )
        .ok_or(miette::miette!("No path found"))?;

        Ok(path.1)
    }

    fn part2(input: &Self::Input<'_>) -> Result<String> {
        let count = COUNT.load(Ordering::Relaxed);
        let mut memory = HashSet::<Coord>::from_iter(input.iter().copied().take(count));
        let limit = LIMIT.load(Ordering::Relaxed) as isize;
        let mut path = HashSet::<Coord>::from_iter(
            pathfinding::directed::astar::astar(
                &Coord(0, 0),
                |&c| {
                    four_neighbors(c, (limit + 1, limit + 1))
                        .filter(|c| !memory.contains(c))
                        .map(|c| (c, 1))
                },
                |_| 1,
                |&c| c == Coord(limit, limit),
            )
            .ok_or(miette::miette!("No path found"))?
            .0,
        );
        let byte = input
            .iter()
            .skip(count)
            .find(|&c| {
                memory.insert(*c);
                if path.contains(c) {
                    let this_path = pathfinding::directed::astar::astar(
                        &Coord(0, 0),
                        |&c| {
                            four_neighbors(c, (limit + 1, limit + 1))
                                .filter(|c| !memory.contains(c))
                                .map(Coord::from)
                                .map(|c| (c, 1))
                        },
                        |_| 1,
                        |&c| c == Coord(limit, limit),
                    );
                    if let Some((this_path, _)) = this_path {
                        path = HashSet::from_iter(this_path);
                        false
                    } else {
                        true
                    }
                } else {
                    false
                }
            })
            .ok_or(miette::miette!("No path found"))?;

        Ok(format!("{},{}", byte.0, byte.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            preamble = {
                LIMIT.store(6, Ordering::Relaxed);
                COUNT.store(12, Ordering::Relaxed);
            };
            input = indoc::indoc! {"
                5,4
                4,2
                4,5
                3,0
                2,1
                6,3
                2,4
                1,5
                0,6
                3,3
                2,6
                5,1
                1,2
                5,5
                2,5
                6,5
                1,4
                0,4
                6,4
                1,1
                6,1
                1,0
                0,5
                1,6
                2,0
            "};
            part1 = 22;
            part2 = "6,1";
    }

    prod_case! {
        part1 = 416;
        part2 = "50,23";
    }
}
