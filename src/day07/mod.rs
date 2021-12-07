use anyhow::Result;

use crate::Runner;

pub struct Day;

fn mean(l: &[usize]) -> f64 {
    let sum = l.iter().sum::<usize>();
    (sum as f64) / (l.len() as f64)
}

fn median(l: &[usize]) -> usize {
    let len = l.len();
    let mid = len / 2;
    if len % 2 == 0 {
        mean(&l[(mid - 1)..(mid + 1)]).round() as _
    } else {
        l[mid]
    }
}

impl Runner for Day {
    type Input = Vec<usize>;
    type Output = usize;

    fn day() -> usize {
        7
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .trim()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect())
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        let idx = median(input) - 1;
        Ok(input
            .iter()
            .copied()
            .map(|v| (v as isize - idx as isize).abs() as usize)
            .sum())
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let floor = mean(input).floor() as usize;
        let ceil = mean(input).ceil() as usize;
        Ok([floor, ceil]
            .iter()
            .copied()
            .map(|idx| {
                input
                    .iter()
                    .copied()
                    .map(|v| {
                        let n = (v as isize - idx as isize).abs() as usize;
                        n * (n + 1) >> 1
                    })
                    .sum()
            })
            .min()
            .unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "16,1,2,0,4,2,7,1,2,14";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(37, Day::part1(&input)?);
        assert_eq!(168, Day::part2(&input)?);
        Ok(())
    }
}
