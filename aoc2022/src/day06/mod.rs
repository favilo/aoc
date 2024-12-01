use color_eyre::Result;

use crate::Runner;

pub struct Day;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy)]
struct Set(u64);

impl Set {
    fn toggle(mut self, c: u8) -> Self {
        debug_assert!(c.is_ascii_lowercase());
        self.0 ^= 1 << (c - b'a');
        self
    }

    fn is_unique(self, ones: usize) -> bool {
        TryInto::<usize>::try_into(self.0.count_ones()).unwrap() == ones
    }
}

impl Runner for Day {
    type Input<'input> = &'input str;

    fn day() -> usize {
        6
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(input)
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(get_index(input.as_bytes(), 4))
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        Ok(get_index(input.as_bytes(), 14))
    }
}

fn get_index(input: &[u8], window_size: usize) -> usize {
    let mut set = input
        .iter()
        .take(window_size - 1)
        .fold(Set::default(), |set, &c| set.toggle(c));
    input
        .windows(window_size)
        .enumerate()
        .find_map(|(i, slice)| {
            set = set.toggle(*slice.last().unwrap());
            if set.is_unique(window_size) {
                return Some(i + window_size);
            }
            set = set.toggle(*slice.first().unwrap());
            None
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::helpers::{prod_case, sample_case};

    use super::*;

    sample_case! {
        sample1 =>
            input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
            part1 = 7;
            part2 = 19;
    }

    sample_case! {
        sample2 =>
            input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
            part1 = 5;
            part2 = 23;
    }

    sample_case! {
        sample3 =>
            input = "nppdvjthqldpwncqszvftbrmjlhg";
            part1 = 6;
            part2 = 23;
    }

    prod_case! {
        part1 = 1779;
        part2 = 2635;
    }
}
