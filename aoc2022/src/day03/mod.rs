use std::fmt::{Debug, Write};

use byte_set::ByteSet;
use color_eyre::Result;
use heapless::Vec;

use crate::Runner;

pub struct Day;

#[derive(Clone, Copy, PartialEq, Ord, PartialOrd, Hash, Eq, hash32_derive::Hash32)]
#[repr(transparent)]
pub struct Priority(u8);

impl Debug for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.0 {
            c @ 1..=26 => b'a' + c - 1,
            c @ 27..=52 => b'A' + c - 27,
            c => panic!("Bad data {c}"),
        } as char;
        f.write_char(c)
        // f.debug_tuple("Priority").field(&c).finish()
    }
}

impl Priority {
    #[allow(dead_code)]
    fn from_ascii(c: u8) -> Self {
        Self(match c {
            c @ b'a'..=b'z' => c - b'a' + 1,
            c @ b'A'..=b'Z' => c - b'A' + 27,
            c => panic!("Not letter {c}"),
        })
    }

    #[allow(dead_code)]
    fn from_ascii_branchless(c: u8) -> Self {
        // Shamelessly stolen from @phone
        // Lowercase letters will always have bit 5 set. The ranges are:
        // A(hex: 41, binary: 1000001) - Z(hex: 5a, binary: 1011010)
        // a(hex: 61, binary: 1100001) - z(hex: 7a, binary: 1111010)
        //
        // Test the 5th bit. This gives us an unsigned quantity either 0 or 1

        // size_t test_lower = (x >> 5) & 1;
        // int64_t ret = 0;
        // If the test passed, unsigned 0 minus the test will underflow and produce a
        // 64 bit word filled with 1s, which can be safely bitwise ANDed with whatever
        // you want
        // ret += (0UL - test_lower) & ((x - 'a') + 1);
        // ret += (0UL - (!test_lower) & ((x - 'A') + 27));
        // return ret;
        let test_lower = (c >> 5) & 1;
        let lower_mask = 0u8.wrapping_sub(test_lower);
        let p =
            (lower_mask & (c.wrapping_sub(b'a') + 1)) + (!lower_mask & (c.wrapping_sub(b'A') + 27));
        Self(p)
    }

    fn to_inner(self) -> u8 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Set(u64);

#[allow(dead_code)] // Historical evidence
impl Set {
    fn union(self, other: Self) -> Self {
        Set(self.0 | other.0)
    }

    fn intersection(self, other: Self) -> Self {
        Set(self.0 & other.0)
    }

    fn only_item(self) -> usize {
        debug_assert_eq!(1, self.0.count_ones());
        self.0.trailing_zeros() as usize
    }
}

impl Debug for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        (0..64).for_each(|i| {
            if self.0 & (1 << i) != 0 {
                write!(f, " {:?}, ", Priority(i)).unwrap();
            }
        });
        write!(f, "}}")
    }
}

impl FromIterator<u8> for Set {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut set = Self(0);
        iter.into_iter()
            .map(|b| {
                assert!(b < 64);
                b
            })
            .for_each(|b| set.0 |= 1u64 << b);
        set
    }
}

impl Runner for Day {
    type Input<'input> = Vec<[ByteSet; 2], 300>;

    fn day() -> usize {
        3
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(input
            .lines()
            .map(|line| {
                let line = line.trim();
                let (first, second) = line.split_at(line.len() / 2);
                [first, second].map(|line| line.bytes().collect::<ByteSet>())
            })
            .collect())
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .into_iter()
            .map(|[left, right]| -> usize {
                let intersection = left.intersection(*right);
                Priority::from_ascii(intersection.into_iter().next().unwrap()).to_inner() as usize
            })
            .sum())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let answer = input
            .chunks_exact(3)
            .map(|l| {
                let [first, second, last] = l else {panic!("Bad number of packs")};
                let first = first[0].union(first[1]);
                let second = second[0].union(second[1]);
                let last = last[0].union(last[1]);
                let intersect = first.intersection(second);
                Priority::from_ascii(intersect.intersection(last).into_iter().next().unwrap())
                    .to_inner() as usize
            })
            .sum::<usize>();
        Ok(answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{sample_case, prod_case};

    sample_case! {
        sample1 => input = "\
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw";
        part1 = 157;
        part2 = 70;
    }

    #[test]
    fn branchless() -> Result<()> {
        assert_eq!(Priority::from_ascii_branchless(b'a'), Priority(1));
        assert_eq!(Priority::from_ascii_branchless(b'A'), Priority(27));
        Ok(())
    }

    prod_case! {
        part1 = 7763;
        part2 = 2569;
    }
}
