use std::collections::HashMap;

use color_eyre::Result;

use crate::Runner;

pub struct Day;

impl Runner for Day {
    type Input<'input> = &'input str;

    fn day() -> usize {
        1
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Ok(input)
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        Ok(input
            .lines()
            .map(|line| {
                line.chars()
                    .filter(|c| c.is_ascii_digit())
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<_>>()
            })
            .map(|digits| 10 * digits[0] + digits[digits.len() - 1])
            .sum())
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let mut map = HashMap::new();
        map.insert("one", 1);
        map.insert("two", 2);
        map.insert("three", 3);
        map.insert("four", 4);
        map.insert("five", 5);
        map.insert("six", 6);
        map.insert("seven", 7);
        map.insert("eight", 8);
        map.insert("nine", 9);
        map.insert("1", 1);
        map.insert("2", 2);
        map.insert("3", 3);
        map.insert("4", 4);
        map.insert("5", 5);
        map.insert("6", 6);
        map.insert("7", 7);
        map.insert("8", 8);
        map.insert("9", 9);
        Ok({
            input
                .lines()
                .map(|line| {
                    let min = map
                        .keys()
                        .filter_map(|k| line.find(k).map(|idx| (idx, map[k])))
                        .min_by_key(|t| t.0)
                        .unwrap();
                    let max = map
                        .keys()
                        .filter_map(|k| line.rfind(k).map(|idx| (idx, map[k])))
                        .max_by_key(|t| t.0)
                        .unwrap();
                    (min.1, max.1)
                })
                .map(|(ten, one)| 10 * ten + one)
                .sum()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input1 = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
            part1 = 142;
            input2 = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
            part2 = 281;
    }

    prod_case! {
        part1 = 54644;
        part2 = 53348;
    }
}
