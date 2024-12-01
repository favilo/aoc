use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
    iter::once,
    str::FromStr,
};

use color_eyre::{eyre::eyre, Result};
use itertools::Itertools;

use crate::{utils::parse_int, Runner};

pub struct Day;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = color_eyre::eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => Err(eyre!("Bad input"))?,
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Movement(Direction, isize);

impl Movement {
    fn perform(self, h: (isize, isize)) -> impl Iterator<Item = (isize, isize)> {
        let Movement(d, m) = self;
        let (x, y) = h;
        (1..=m).into_iter().map(move |i| match d {
            Direction::Up => (x, y + i),
            Direction::Down => (x, y - i),
            Direction::Left => (x - i, y),
            Direction::Right => (x + i, y),
        })
    }
}

fn next_to(h: (isize, isize), t: (isize, isize)) -> bool {
    (h.0 - t.0).abs() < 2 && (h.1 - t.1).abs() < 2
}

fn follow_once(h: (isize, isize), t: (isize, isize)) -> (isize, isize) {
    let (xh, yh) = h;
    let (xt, yt) = t;

    let (dx, dy) = (xh - xt, yh - yt);

    if next_to(h, t) {
        return t;
    }

    (xt + dx.signum(), yt + dy.signum())
}

fn follow(
    heads: impl IntoIterator<Item = (isize, isize)>,
    t: (isize, isize),
    set: &mut HashSet<(isize, isize)>,
) -> ((isize, isize), (isize, isize)) {
    let mut head = t;
    let mut tail = t;
    heads.into_iter().for_each(|h| {
        head = h;
        tail = follow_once(head, tail);
        set.insert(tail);
    });
    (head, tail)
}

#[allow(dead_code)]
fn print_grid(chain: &[(isize, isize); 10], set: &HashSet<(isize, isize)>) {
    let mut positions: HashMap<(isize, isize), usize> = HashMap::new();
    chain.iter().enumerate().for_each(|(i, t)| {
        if !positions.contains_key(t) {
            positions.insert(*t, i);
        }
    });
    let letters = (0..10)
        .zip(once('H').chain('1'..='9'))
        .collect::<HashMap<_, _>>();

    (-5..16).rev().for_each(|row| {
        println!();
        (-12..16).for_each(|col| {
            let key = (col, row);
            if positions.contains_key(&key) {
                print!("{}", letters[&positions[&key]]);
            } else if set.contains(&key) {
                print!("#");
            } else if key == (0, 0) {
                print!("s");
            } else {
                print!(".");
            }
        })
    });
    println!()
}

fn follow_chain(
    chain: &mut [(isize, isize); 10],
    heads: impl IntoIterator<Item = (isize, isize)>,
    set: &mut HashSet<(isize, isize)>,
) {
    heads.into_iter().for_each(|h| {
        chain[0] = h;
        let cell = &Cell::from_mut(&mut chain[..]);
        let slice_cell = cell.as_slice_of_cells();
        slice_cell.iter().tuple_windows().for_each(|(h, t)| {
            t.set(follow_once(h.get(), t.get()));
        });
        set.insert(chain[9]);
    });
}

impl Runner for Day {
    type Input<'input> = Vec<Movement>;

    fn day() -> usize {
        9
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(input
            .lines()
            .filter(|l| !l.is_empty())
            .map(str::trim)
            .filter_map(|l| l.split_once(' '))
            .map(|(d, m)| {
                let d = d.parse().unwrap();
                let m = parse_int(m.as_bytes()).try_into().unwrap();
                Movement(d, m)
            })
            .collect())
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut head: (isize, isize) = (0, 0);
        let mut tail: (isize, isize) = (0, 0);
        let mut set = HashSet::new();

        input.iter().for_each(|m| {
            let heads = m.perform(head);
            (head, tail) = follow(heads, tail, &mut set);
        });

        Ok(set.len())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut chain = [(0, 0); 10];
        let mut set = HashSet::new();

        input.iter().for_each(|m| {
            let heads = m.perform(chain[0]);

            follow_chain(&mut chain, heads, &mut set);
        });
        set.insert(chain[9]);
        Ok(set.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "
                    R 4
                    U 4
                    L 3
                    D 1
                    R 4
                    D 1
                    L 5
                    R 2
                    ";
            part1 = 13;
            part2 = 1;
    }

    sample_case! {
        sample2 =>
            input = "
                    R 5
                    U 8
                    L 8
                    D 3
                    R 17
                    D 10
                    L 25
                    U 20
                    ";
            part1 = 88;
            part2 = 36;
    }

    prod_case! {
        part1 = 5960;
        part2 = 2327;
    }
}
