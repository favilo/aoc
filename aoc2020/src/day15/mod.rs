use anyhow::Result;

use crate::Runner;

pub struct Day15;

fn game(input: &<Day15 as Runner>::Input, num: usize) -> Result<usize> {
    let mut seen = vec![None; num];
    input
        .iter()
        .enumerate()
        .for_each(|(idx, &i)| seen[i] = Some(idx));
    let mut last = *input.last().unwrap();
    let mut last_seen = None;
    (input.len()..num).for_each(|i| {
        last = (i - 1) - last_seen.unwrap_or(i - 1);
        last_seen = std::mem::replace(&mut seen[last], Some(i));
    });
    // println!(
    //     "Highest seen: {:?}",
    //     seen.iter()
    //         .enumerate()
    //         .rev()
    //         .filter(|(_, o)| o.is_some())
    //         .next()
    // );

    Ok(last)
}

impl Runner for Day15 {
    type Input = Vec<usize>;
    type Output = usize;

    fn day() -> usize {
        15
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .trim()
            .split(',')
            .map(|i| i.parse().unwrap())
            .collect())
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        game(input, 2020)
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        game(input, 30000000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "0,3,6";

        let input = Day15::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(436, Day15::part1(&input)?);
        assert_eq!(175594, Day15::part2(&input)?);
        Ok(())
    }
}
