use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::opt,
    multi::many1,
    sequence::terminated,
    IResult,
};
use num::Integer;

use crate::Runner;

#[derive(Debug, Clone)]
pub struct Schedule(u64, Vec<(usize, u64)>);

fn parse_input(input: &str) -> IResult<&str, Schedule> {
    let (input, bus) = terminated(digit1, multispace0)(input)?;
    let (input, sched) = terminated(
        many1(terminated(alt((digit1, tag("x"))), opt(char(',')))),
        multispace0,
    )(input)?;

    Ok((
        input,
        Schedule(
            bus.parse().unwrap(),
            sched
                .into_iter()
                .enumerate()
                .filter_map(|(i, n)| match n {
                    "x" => None,
                    n => Some((i, n.parse().unwrap())),
                })
                .collect(),
        ),
    ))
}

pub struct Day13;

impl Runner for Day13 {
    type Input = Schedule;
    type Output = u64;

    fn day() -> usize {
        13
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(parse_input(input).unwrap().1)
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        let bus = input.0;
        let (bus, time) = input
            .1
            .iter()
            .map(|(_, sched)| (sched, (((bus / sched) + 1) * sched) - bus))
            .min_by_key(|t| t.1)
            .unwrap();
        Ok(time * bus)
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let time = input.1.iter().map(|&(i, o)| (i as u64, o)).fold(
            (0, 1),
            |(mut time, mut delta), (offset, b)| {
                // Find the first time that matches for the schedules so far seen
                while (time + offset) % b != 0 {
                    time += delta;
                }
                // When you find a time that matches, we need to lock the delta
                // to always jump by effectively the product of all the
                // schedules that came before
                delta = delta.lcm(&b);
                (time, delta)
            },
        );

        Ok(time.0)
    }
}

// These didn't go as quickly as I'd thought they might. So just sticking with my original solution.
// fn chinese_remainder(residues: &[u64], modulii: &[u64]) -> Option<u64> {
//     let prod: u64 = modulii.iter().product();
//     let mut sum = 0;
//     residues
//         .iter()
//         .zip(modulii)
//         .for_each(|(&residue, &modulus)| {
//             let p = prod / modulus;
//             sum += residue * mod_inv(p as i64, modulus as i64).unwrap() as u64 * p;
//         });
//     Some(sum % prod)
// }

// fn mod_inv(x: i64, n: i64) -> Option<i64> {
//     let gcd = x.extended_gcd(&n);
//     if gcd.gcd == 1 {
//         Some((gcd.x % n + n) % n)
//     } else {
//         None
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "939\n\
                     7,13,x,x,59,x,31,19";

        let input = Day13::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(295, Day13::part1(&input)?);
        assert_eq!(1068781, Day13::part2(&input)?);
        Ok(())
    }

    #[test]
    fn sample2() -> Result<()> {
        let input = "939\n\
                     17,x,13,19";

        let input = Day13::get_input(input)?;
        println!("{:?}", input);
        // assert_eq!(295, Day13::part1(&input)?);
        assert_eq!(3417, Day13::part2(&input)?);
        Ok(())
    }

    #[test]
    fn sample3() -> Result<()> {
        let input = "939\n\
                     1789,37,47,1889";

        let input = Day13::get_input(input)?;
        println!("{:?}", input);
        // assert_eq!(295, Day13::part1(&input)?);
        assert_eq!(1202161486, Day13::part2(&input)?);
        Ok(())
    }

    #[test]
    fn sample4() -> Result<()> {
        let input = "939\n\
                     67,7,59,61";

        let input = Day13::get_input(input)?;
        println!("{:?}", input);
        // assert_eq!(295, Day13::part1(&input)?);
        assert_eq!(754018, Day13::part2(&input)?);
        Ok(())
    }
}
