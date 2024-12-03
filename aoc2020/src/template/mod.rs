use anyhow::Result;

use crate::Runner;

pub struct DayXX;
impl Runner for DayXX {
    type Input = Vec<()>;
    type Output = usize;

    fn day() -> usize {
        0 // XX
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

        let input = DayXX::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(436, DayXX::part1(&input)?);
        assert_eq!(175594, DayXX::part2(&input)?);
        Ok(())
    }
}
