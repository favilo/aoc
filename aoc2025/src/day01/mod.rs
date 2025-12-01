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

    /// Returns new position and how many times we passed zero
    pub fn rotate_pass_zero(&self, start: usize) -> (usize, usize) {
        let mut count = 0;
        match self.direction {
            Direction::Left => {
                let mut after_rotation = start as isize - self.value as isize;
                count = self.value / 100;
                if start != 0 && (after_rotation < 0 || after_rotation == 0) {
                    count += 1;
                }
                (after_rotation.rem_euclid(100) as usize, count)
            }
            Direction::Right => {
                let mut after_rotation = start + self.value;
                count = self.value / 100;
                if start != 0 && (after_rotation > 100 || after_rotation % 100 == 0) {
                    count += 1;
                }
                (after_rotation.rem_euclid(100), count)
            }
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
        Ok(input
            .lines()
            .map(|line| {
                let mut chars = line.chars();
                let dir = Direction::from_char(chars.next().unwrap()).unwrap();
                let value: usize = chars.collect::<String>().parse().unwrap();
                Turn {
                    direction: dir,
                    value,
                }
            })
            .collect())
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let mut start = 50;
        let mut count_zero = 0;
        input.iter().for_each(|turn| {
            start = turn.rotate(start);
            if start == 0 {
                count_zero += 1;
            }
        });

        Ok(count_zero)
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut start = 50;
        let mut count_zero = 0;
        input.iter().for_each(|turn| {
            let (new_start, passed) = dbg!(turn).rotate_pass_zero(start);
            start = dbg!(new_start);
            count_zero += dbg!(passed);
        });

        Ok(count_zero)
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

    sample_case! {
        sample_left_to_zero =>
            input = indoc::indoc! {"
                L50
            "};
            part1 = 1;
            part2 = 1;
    }

    sample_case! {
        sample_right_to_zero =>
            input = indoc::indoc! {"
                R50
            "};
            part1 = 1;
            part2 = 1;
    }

    sample_case! {
        sample_right_left =>
            input = indoc::indoc! {"
                R50
                L400
            "};
            part1 = 2;
            part2 = 5;
    }

    sample_case! {
        sample_left_right =>
            input = indoc::indoc! {"
                L50
                R400
            "};
            part1 = 2;
            part2 = 5;
    }

    sample_case! {
        sample_right_small_bits =>
            input = indoc::indoc! {"
                R50
                L1
                R2
                L2
            "};
            part1 = 1;
            part2 = 3;
    }

    prod_case! {
        part1 = 1105;
        part2 = 0;
    }
}
