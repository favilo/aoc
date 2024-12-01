use std::{
    collections::HashMap,
    fmt::{Debug, Write},
};

use color_eyre::Result;
use heapless::Vec as SmallVec;
use itertools::Itertools;
use once_cell::sync::Lazy;

use crate::Runner;

pub struct Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Right,
    Left,
}

#[derive(Clone)]
struct Shape(SmallVec<u8, 4>);

impl Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .rev()
            .map(|r| {
                (0..7)
                    .rev()
                    .map(|i| (r >> i) & 1 != 0)
                    .map(|b| -> std::fmt::Result { f.write_char(if b { '@' } else { '.' }) })
                    .collect::<std::fmt::Result>()?;
                f.write_str("\n")
            })
            .collect::<std::fmt::Result>()
    }
}

impl Shape {
    fn shift(&mut self, dir: Dir, grid: &Grid, height: usize) -> &mut Self {
        if self.leftmost() == 0 && dir == Dir::Right || self.rightmost() == 6 && dir == Dir::Left {
            return self;
        }
        let mut new = self.clone();
        match dir {
            Dir::Right => new.0.iter_mut().for_each(|b| *b >>= 1),
            Dir::Left => new.0.iter_mut().for_each(|b| *b <<= 1),
        }
        if !grid.shape_fits(&new, height) {
            return self;
        }
        *self = new;
        self
    }

    fn leftmost(&self) -> u32 {
        self.0.iter().map(|b| b.trailing_zeros()).min().unwrap()
    }

    fn rightmost(&self) -> u32 {
        self.0.iter().map(|b| 7 - b.leading_zeros()).max().unwrap()
    }

    fn height(&self) -> usize {
        self.0.len()
    }
}

static SHAPES: Lazy<SmallVec<Shape, 5>> = Lazy::new(|| {
    SmallVec::from_slice(&[
        Shape(SmallVec::from_slice(&[0b1111]).unwrap()),
        Shape(SmallVec::from_slice(&[0b010, 0b111, 0b010]).unwrap()),
        Shape(SmallVec::from_slice(&[0b111, 0b001, 0b001]).unwrap()),
        Shape(SmallVec::from_slice(&[0b1, 0b1, 0b1, 0b1]).unwrap()),
        Shape(SmallVec::from_slice(&[0b11, 0b11]).unwrap()),
    ])
    .unwrap()
});

#[derive(Clone)]
pub struct Grid(Vec<u8>);

impl Grid {
    fn new() -> Self {
        Self(vec![0])
    }

    fn process_shape<I>(
        &mut self,
        shape: &Shape,
        dirs: &mut I,
        num_dirs: usize,
        old_state: State,
    ) -> State
    where
        I: Iterator<Item = Dir>,
    {
        let State {
            dir_idx: mut old_dir_idx,
            shape_idx: old_shape_idx,
            ..
        } = old_state;
        let mut y = self.height() + 3;
        let mut shape = shape.clone();
        while shape.rightmost() < 4 {
            shape.shift(Dir::Left, self, y);
        }
        while self.shape_fits(&shape, y) {
            let Some(d) = dirs.next() else { panic!("Ran out of infinite supply") };
            old_dir_idx += 1;
            shape.shift(d, self, y);
            if y == 0 {
                break;
            }
            if self.shape_fits(&shape, y - 1) {
                y -= 1;
            } else {
                break;
            }
        }
        if self.capacity() <= y + shape.height() {
            self.0.extend_from_slice(&[0, 0, 0, 0, 0]);
        }

        shape.0.iter().enumerate().for_each(|(i, r)| {
            self.0[y + i] |= r;
        });

        State {
            visible: self.visible_rows(),
            shape_idx: (old_shape_idx + 1) % SHAPES.len(),
            dir_idx: old_dir_idx % num_dirs,
        }
    }

    fn shape_fits(&self, shape: &Shape, h: usize) -> bool {
        let height = self.capacity();
        if h >= height {
            return true;
        }
        let height_diff = height - h;
        (0..height_diff.min(shape.height())).all(|i| {
            let shape_row = shape.0[i];
            if i + h >= height - 1 {
                return true;
            }
            let row = self.0[(i + h)];
            shape_row.count_ones() + row.count_ones() == (shape_row | row).count_ones()
        })
    }

    fn height(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .find_map(|(i, &r)| (r == 0).then_some(i))
            .unwrap_or_else(|| self.capacity())
    }

    fn capacity(&self) -> usize {
        self.0.len()
    }

    fn visible_rows(&self) -> Vec<u8> {
        let (rows, _remaining_cols) = self.0.iter().copied().rev().filter(|&b| b != 0).fold(
            (Vec::new(), Vec::from_iter(0..7)),
            |(mut rows, mut cols), row| {
                if !cols.is_empty() {
                    rows.push(row);
                } else {
                    return (rows, vec![]);
                }
                cols.drain_filter(|column| row & (1 << *column) != 0);
                (rows, cols)
            },
        );

        rows
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct State {
    visible: Vec<u8>,
    shape_idx: usize,
    dir_idx: usize,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .rev()
            .map(|r| {
                f.write_str("|")?;
                (0..7)
                    .rev()
                    .map(|i| (r >> i) & 1 != 0)
                    .map(|b| -> std::fmt::Result { f.write_char(if b { '#' } else { '.' }) })
                    .collect::<std::fmt::Result>()?;
                f.write_str("|\n")
            })
            .collect::<std::fmt::Result>()?;
        f.write_str("|-------|\n")
    }
}

impl Runner for Day {
    type Input<'input> = Vec<Dir>;

    fn day() -> usize {
        17
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(input
            .chars()
            .filter(|&c| c == '<' || c == '>')
            .map(|c| match c {
                '>' => Dir::Right,
                '<' => Dir::Left,
                c => panic!("Wrong letter: {c}"),
            })
            .collect_vec())
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        // highest rock will be len of list, push up
        let mut grid = Grid::new();
        // Repeast directions forever
        let mut directions = input.into_iter().copied().cycle();
        // Repeat shapes forever
        SHAPES.iter().cycle().take(2022).for_each(|s| {
            grid.process_shape(s, &mut directions, input.len(), State::default());
        });
        Ok(grid.height())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let target_shapes = 1_000_000_000_000_usize;
        let mut grid = Grid::new();
        let mut directions = input.into_iter().copied().cycle();
        let mut seen = HashMap::<State, (usize, usize)>::new();
        let mut state = State::default();
        let mut total_shapes = 0;
        let mut cycle = SHAPES.iter().cycle();
        for s in &mut cycle {
            state = grid.process_shape(s, &mut directions, input.len(), state);
            if seen.contains_key(&state) {
                break;
            }
            seen.insert(state.clone(), (grid.height(), total_shapes));
            total_shapes += 1;
        }
        let (old_height, old_shape_count) = seen.get(&state).unwrap();
        let height_gained = grid.height() - old_height;
        let shapes_in_loop = total_shapes - old_shape_count;
        let loop_count = (target_shapes - old_shape_count - 1) / shapes_in_loop;
        let remaining_shapes_to_drop =
            target_shapes - old_shape_count - (loop_count * shapes_in_loop);
        for s in cycle.take(remaining_shapes_to_drop) {
            state = grid.process_shape(s, &mut directions, input.len(), state);
        }

        Ok(grid.height() - 1 + height_gained * (loop_count - 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
            part1 = 3068;
            part2 = 1514285714288;
    }

    prod_case! {
        part1 = 3211;
        part2 = 1589142857183;
    }
}
