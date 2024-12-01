use anyhow::Result;

use crate::Runner;

pub struct Day17;
impl Runner for Day17 {
    type Input = Vec<()>;
    type Output = usize;

    fn day() -> usize {
        17
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        todo!()
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        todo!()
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "0,3,6";

        let input = Day17::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(436, Day17::part1(&input)?);
        assert_eq!(175594, Day17::part2(&input)?);
        Ok(())
    }
}
