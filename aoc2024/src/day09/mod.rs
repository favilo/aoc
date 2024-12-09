use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::repeat;

use aoc_utils::parse::parse_int;
use itertools::Itertools;
use miette::Result;

use crate::Runner;

pub struct Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct File {
    id: u32,
    location: usize,
    size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Empty {
    location: usize,
    size: usize,
}

impl PartialOrd for Empty {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Reverse(self.location).cmp(&Reverse(other.location)))
    }
}

impl Ord for Empty {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone)]
pub struct Disk {
    /// The index of file ids for each block on the disk
    blocks: Vec<Option<u32>>,

    /// The files on disk organized by size and id
    files: Vec<File>,

    /// The empty spaces on the disk
    empties: BinaryHeap<Empty>,
}

impl std::fmt::Debug for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Disk")
            .field_with("blocks", |f| {
                f.write_str(&format!(
                    "{:?}",
                    self.blocks
                        .iter()
                        .map(|b| b.map(|b| format!("{b}")).unwrap_or(String::from(" ")))
                        .collect::<Vec<_>>()
                ))
            })
            .field("files", &self.files)
            .field("empties", &self.empties)
            .finish()
    }
}

impl Disk {
    pub fn from_input(input: &str) -> Self {
        let mut is_file = true;
        let mut id = 0;
        let mut blocks = Vec::with_capacity(1024);
        let mut files = Vec::with_capacity(1024);
        let mut empties = BinaryHeap::new();
        for c in input.trim().bytes() {
            let size = parse_int(&[c]);
            let block_data = if is_file {
                let this = Some(id);
                files.push(File {
                    id,
                    size,
                    location: blocks.len(),
                });
                id += 1;
                this
            } else {
                empties.push(Empty {
                    location: blocks.len(),
                    size,
                });
                None
            };
            blocks.extend(repeat(block_data).take(size));
            is_file = !is_file;
        }
        let mut disk = Self {
            blocks,
            files,
            empties,
        };
        disk.merge_empties();
        disk
    }

    pub fn compact(&mut self) {
        let mut idx = 0;
        while idx < self.blocks.len() {
            if self.blocks[idx].is_none() {
                let next = self.pop_non_empty_block().unwrap();
                self.set(idx, next);
            }
            idx += 1;
        }
    }

    pub fn compact_files(&mut self) {
        let mut files = std::mem::take(&mut self.files);
        for this_file in files.iter_mut().rev() {
            log::debug!("Examining: {this_file:?}");
            log::debug!("Looking for empty with size {}", this_file.size);
            log::debug!("Empties: {:?}", self.empties);
            let Some(mut this_empty) = self.fetch_empty(this_file.size, this_file.location) else {
                continue;
            };
            log::debug!("Moving from {:?} to {:?}", this_file, this_empty);
            self.move_file(this_empty, this_file);

            this_empty.location += this_file.size;
            this_empty.size -= this_file.size;

            if this_empty.size > 0 {
                log::debug!("Pushing back: {this_empty:?}");
                self.empties.push(this_empty);
            }
            self.merge_empties();
        }

        std::mem::swap(&mut self.files, &mut files);
    }

    fn move_file(&mut self, this_empty: Empty, this_file: &mut File) {
        let (before, after) = self
            .blocks
            .split_at_mut(this_empty.location + this_file.size);
        let after_start_idx = this_file.location - before.len();
        before[this_empty.location..]
            .swap_with_slice(&mut after[after_start_idx..][..this_file.size]);
        log::debug!("Disk after copy blocks: {:?}", self.blocks);

        self.empties.push(Empty {
            location: this_file.location,
            size: this_file.size,
        });

        this_file.location = this_empty.location;
    }

    fn pop_non_empty_block(&mut self) -> Option<u32> {
        loop {
            if let Some(next) = self.blocks.pop()? {
                self.trim();
                return Some(next);
            }
        }
    }

    fn trim(&mut self) {
        while self.blocks.last().unwrap().is_none() {
            self.blocks.pop();
        }
    }

    fn set(&mut self, idx: usize, data: u32) {
        if idx == self.blocks.len() {
            self.blocks.push(Some(data));
            return;
        }
        self.blocks[idx] = Some(data);
    }

    fn check_sum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .filter_map(|(i, b)| b.map(|b| i * b as usize))
            .sum()
    }

    fn fetch_empty(&mut self, size: usize, before: usize) -> Option<Empty> {
        let empties = std::mem::take(&mut self.empties);
        let mut found = None;
        self.empties.extend(empties.into_iter_sorted().filter(|e| {
            if e.size >= size && e.location < before && found.is_none() {
                found = Some(*e);
                false
            } else {
                true
            }
        }));
        found
    }

    fn merge_empties(&mut self) {
        let old_empties = std::mem::take(&mut self.empties);
        log::debug!("Merging before: {old_empties:?}");

        let mut iter = old_empties.into_iter_sorted().filter(|e| e.size > 0);
        let Some(mut this) = iter.next() else {
            return;
        };

        iter.flat_map(|next| {
            if this.location + this.size >= next.location {
                this = Empty {
                    location: this.location,
                    size: next.location + next.size - this.location,
                };
                None
            } else {
                let that = Some(this);
                this = next;
                that
            }
        })
        .for_each(|e| self.empties.push(e));
        self.empties.push(this);
        log::debug!("Merging after: {:?}", self.empties);
    }
}

impl Runner for Day {
    type Input<'input> = Disk;

    fn day() -> usize {
        9
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(Disk::from_input(input))
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut disk = input.clone();
        disk.compact();

        Ok(disk.check_sum())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut disk = input.clone();
        disk.compact_files();

        log::debug!("Disk after compaction: {:?}", disk);
        Ok(disk.check_sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "2333133121414131402";
            part1 = 1928;
            part2 = 2858;
    }

    sample_case! {
        sample2 =>
            input = "2929293030303030309";
            part1 = 1928;
            part2 = 2642;
    }

    prod_case! {
        part1 = 6367087064415;
        part2 = 201684;
    }
}
