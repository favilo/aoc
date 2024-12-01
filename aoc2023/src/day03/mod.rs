use std::collections::{HashMap, HashSet};

use color_eyre::Result;
use itertools::Itertools;

use crate::{
    utils::{eight_neighbors, four_neighbors},
    Runner,
};

pub struct Day;

impl Runner for Day {
    type Input<'input> = (
        (usize, usize),
        Vec<usize>,
        HashMap<(usize, usize), usize>,
        HashMap<char, Vec<(usize, usize)>>,
    );

    fn day() -> usize {
        3
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        let width = input.lines().next().unwrap().chars().count();
        let height = input.lines().count();

        let lines: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut current = None;
        let mut current_idx = vec![];

        let mut map = HashMap::new();
        let mut symbols = HashMap::<char, Vec<(usize, usize)>>::new();
        let mut ids = vec![];

        for row in 0..height {
            for col in 0..width {
                let c = lines[row][col];
                match c {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        if let Some(ref mut curr) = current {
                            *curr = *curr * 10 + c as usize - '0' as usize;
                        } else {
                            current = Some(c as usize - '0' as usize);
                        }
                        current_idx.push((row, col));
                    }
                    c => {
                        if let Some(curr) = current {
                            let idx = ids.len();
                            ids.push(curr);
                            current_idx.drain(..).for_each(|(row, col)| {
                                map.insert((row, col), idx);
                            });
                            current = None;
                        }
                        if c != '.' {
                            symbols.entry(c).or_default().push((row, col));
                        }
                    }
                }
            }
        }
        Ok(((height, width), ids, map, symbols))
    }

    fn part1((shape, ref ids, ref map, ref symbols): &Self::Input<'_>) -> Result<usize> {
        let mut set = HashSet::<usize>::default();
        symbols.iter().for_each(|(sym, idx)| {
            // println!("{}: {:?}", sym, idx);
            for (row, col) in idx {
                eight_neighbors((*row, *col), *shape).for_each(|idx| {
                    if map.contains_key(&idx) {
                        set.insert(*map.get(&idx).unwrap());
                    }
                })
            }
        });
        Ok(set.into_iter().map(|idx| ids[idx]).sum())
    }

    fn part2((shape, ref ids, ref map, ref symbols): &Self::Input<'_>) -> Result<usize> {
        let maybe_gears = symbols.get(&'*').unwrap();
        let gears = maybe_gears
            .iter()
            .filter_map(|&idx| {
                let neighbors = eight_neighbors(idx, *shape)
                    .into_iter()
                    .filter_map(|id| map.get(&id).and_then(|v| Some(v)))
                    .collect::<HashSet<_>>();
                (neighbors.len() == 2)
                    .then_some(neighbors.into_iter().map(|id| ids[*id]).product::<usize>())
            })
            .collect_vec();
        Ok(gears.into_iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
            part1 = 4361;
            part2 = 467835;
    }

    prod_case! {
        part1 = 540212;
        part2 = 87605697;
    }
}
