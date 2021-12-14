use std::collections::HashSet;

use anyhow::Result;
use itertools::iproduct;
use ndarray::{Array2, Axis};
use nom::{
    character::complete::{multispace0, one_of},
    combinator::map,
    multi::many1,
    sequence::terminated,
    IResult,
};

use crate::{utils::parse_int, Runner};

fn parse_input<'a>(input: &'a [u8]) -> IResult<&'a [u8], Vec<usize>> {
    let r = terminated(
        many1(map(one_of("0123456789"), |s| parse_int(&[s as u8]))),
        multispace0,
    )(input);
    r
}

pub struct Day;

impl Runner for Day {
    type Input = Array2<usize>;
    type Output = usize;

    fn day() -> usize {
        11
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let mut v = input
            .lines()
            .map(str::as_bytes)
            .map(parse_input)
            .map(Result::unwrap)
            .map(|t| t.1)
            .map(Vec::into_iter)
            .flatten();
        Ok(Array2::from_shape_fn((height, width), |_| {
            v.next().unwrap()
        }))
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        let mut array = input.clone();
        Ok((0..100).map(|_| step(&mut array)).sum())
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let mut array = input.clone();
        let mut steps = 1;
        while input.len() != step(&mut array) {
            steps += 1;
        }
        Ok(steps)
    }
}

#[allow(dead_code)]
fn print_array(array: &Array2<usize>) {
    for row in array.axis_iter(Axis(0)) {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}

fn neighbors((x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    iproduct!(-1..=1, -1..=1)
        .filter(move |&(dx, dy)| dx != 0 || dy != 0)
        .filter(move |(dx, dy)| x as isize + dx >= 0 && y as isize + dy >= 0)
        .map(move |(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
}

fn step(array: &mut Array2<usize>) -> usize {
    array.iter_mut().for_each(|v| *v += 1);
    let mut nines = array
        .indexed_iter()
        .filter(|(_, &v)| v > 9)
        .map(|(idx, _)| idx)
        .collect::<HashSet<_>>();

    let mut flashed = 0;
    let mut visited = HashSet::<(usize, usize)>::new();
    let shape = array.shape().to_vec();

    while visited.len() != nines.len() {
        let mut added = Vec::new();
        let left = nines.difference(&visited).copied().collect::<HashSet<_>>();
        left.iter().for_each(|&idx| {
            visited.insert(idx);
            neighbors(idx)
                .filter(|(x, y)| x < &shape[0] && y < &shape[1])
                .for_each(|idx| {
                    if idx == (2, 2) {}
                    array[idx] += 1;
                    if array[idx] > 9 {
                        added.push(idx);
                    }
                });
        });
        // println!("{:?} -> {:?}", visited, nines);
        nines.extend(added);
        flashed = visited.len();
    }
    array.iter_mut().filter(|v| **v > 9).for_each(|v| {
        *v = 0;
    });
    // array.indexed_iter().map(|((x,y), v));
    // *array = new;

    flashed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(1656, Day::part1(&input)?);
        // assert_eq!(35, Day::part1(&input)?);
        // assert_eq!(175594, Day::part2(&input)?);
        Ok(())
    }

    #[test]
    fn simple_step() -> Result<()> {
        let input = "11111
19991
19191
19991
11111";

        let mut input = Day::get_input(input)?;
        println!("{:?}", input);
        let flashes = step(&mut input);
        let step1 = "34543
40004
50005
40004
34543";
        let step1 = Day::get_input(step1)?;
        println!("\nInput");
        print_array(&input);
        println!("\nStep #1");
        print_array(&step1);
        assert_eq!(step1, input);
        assert_eq!(9, flashes);

        let step2 = "45654
51115
61116
51115
45654";
        let step2 = Day::get_input(step2)?;
        step(&mut input);
        assert_eq!(step2, input);
        Ok(())
    }
}
