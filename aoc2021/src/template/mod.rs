use anyhow::Result;

use crate::Runner;

pub struct Day;

impl Runner for Day {
    type Input = Vec<()>;

    fn day() -> usize {
        0 // FIXME
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        todo!()
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        todo!()
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "0,3,6";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(436, Day::part1(&input)?);
        assert_eq!(175594, Day::part2(&input)?);
        Ok(())
    }
}
