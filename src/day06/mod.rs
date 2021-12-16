use anyhow::Result;

use crate::{utils::parse_int, Runner};

pub struct Day;

#[cfg(feature = "day06_ring")]
mod ring {
    use std::ops::{Index, IndexMut};

    #[derive(Default, Debug, Clone)]
    pub struct Ring([usize; 9], usize);

    impl Ring {
        pub fn advance(&mut self) {
            self.1 += 1;
            self.1 %= 9;
        }
    }

    impl Index<usize> for Ring {
        type Output = usize;
        fn index(&self, index: usize) -> &Self::Output {
            &self.0[(index + self.1) % 9]
        }
    }

    impl IndexMut<usize> for Ring {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.0[(index + self.1) % 9]
        }
    }

    impl<'a> IntoIterator for &'a Ring {
        type Item = &'a usize;

        type IntoIter = std::iter::Chain<std::slice::Iter<'a, usize>, std::slice::Iter<'a, usize>>;

        fn into_iter(self) -> Self::IntoIter {
            let (first, last) = self.0.split_at(self.1);
            last.into_iter().chain(first.into_iter())
        }
    }
}

impl Runner for Day {
    #[cfg(feature = "day06_ring")]
    type Input = ring::Ring;

    #[cfg(not(feature = "day06_ring"))]
    type Input = [usize; 9];

    fn day() -> usize {
        6
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .trim()
            .as_bytes()
            .split(|&c| c == ',' as u8)
            .map(|s| parse_int(s))
            .fold(Default::default(), |mut v, i: usize| {
                v[i] += 1;
                v
            }))
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        let mut v = input.to_owned();
        (0..80).for_each(|_| update_state(&mut v));

        Ok(v.into_iter().sum())
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        let mut v = input.to_owned();
        (0..256).for_each(|_| update_state(&mut v));

        Ok(v.into_iter().sum())
    }
}

#[cfg(feature = "day06_ring")]
fn update_state(ring: &mut ring::Ring) {
    ring.advance();
    ring[6] += ring[8];
}

#[cfg(not(feature = "day06_ring"))]
fn update_state(slice: &mut [usize; 9]) {
    let mut new = [0; 9];
    new[..8].copy_from_slice(&slice[1..]);
    new[8] = slice[0];
    new[6] += slice[0];
    *slice = new;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "3,4,3,1,2";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(5934, Day::part1(&input)?);
        assert_eq!(26984457539, Day::part2(&input)?);
        Ok(())
    }
}
