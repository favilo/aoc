use anyhow::Result;

use crate::Runner;

pub struct Day;

impl Runner for Day {
    type Input = [usize; 9];
    type Output = usize;

    fn day() -> usize {
        6
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .trim()
            .split(",")
            .map(|s| usize::from_str_radix(s, 10).unwrap())
            .fold([0; 9], |mut v, i| {
                v[i] += 1;
                v
            }))
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        let mut v = input.to_owned();
        (0..80).for_each(|_| update_state(&mut v));

        Ok(v.iter().sum())
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let mut v = input.to_owned();
        (0..256).for_each(|_| update_state(&mut v));

        Ok(v.iter().sum())
    }
}

fn update_state(v: &mut [usize; 9]) {
    *v =
        v.iter()
            .copied()
            .enumerate()
            .fold([0; 9], |mut v: [usize; 9], (idx, c): (usize, usize)| {
                if idx == 0 {
                    v[8] += c;
                    v[6] += c;
                } else {
                    v[idx - 1] += c;
                }
                v
            })
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
