use miette::Result;

use aoc_utils::Runner;

pub struct Day;

#[derive(Debug, Clone, Copy)]
pub struct Turn {
    direction: Direction,
    value: usize,
}

impl Turn {
    pub fn rotate(&self, start: usize) -> usize {
        match self.direction {
            Direction::Left => (start as isize - self.value as isize).rem_euclid(100) as usize,
            Direction::Right => (start + self.value).rem_euclid(100),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }
}

impl Runner for Day {
    type Input<'input> = Vec<Turn>;

    #[rustfmt::skip]
    fn day() -> usize {
        1
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(input.lines()
            .map(|line| {
                let mut chars = line.chars();
                let dir = Direction::from_char(chars.next().unwrap()).unwrap();
                let value: usize = chars.collect::<String>().parse().unwrap();
                Turn { direction: dir, value }
            })
            .collect())
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut start = 50;
        let mut count_zero = 0;
        input.iter().for_each(|turn| {
            start = turn.rotate(start);
            if dbg!(start) == 0 {
                count_zero += 1;
            }
        });

        Ok(count_zero)
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = indoc::indoc! {"
                L68
                L30
                R48
                L5
                R60
                L55
                L1
                L99
                R14
                L82
            "};
            part1 = 3;
            part2 = 6;
    }

    prod_case! {
        part1 = 0;
        part2 = 0;
    }
}
