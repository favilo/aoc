use std::cmp::max;

use anyhow::Result;
use ndarray::{Array2, Axis};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, one_of},
    combinator::map,
    error::{convert_error, VerboseError},
    multi::many1,
    sequence::{delimited, terminated, tuple},
    Finish, IResult,
};

use crate::{utils::parse_int, Runner};

#[derive(Debug, Clone, Copy)]
pub enum Fold {
    H(usize),
    V(usize),
}

pub struct Day;

impl Runner<usize, &str> for Day {
    type Input = (Array2<bool>, Vec<Fold>);

    fn day() -> usize {
        13
    }

    fn get_input<'a>(input: &'a str) -> Result<Self::Input> {
        let r = parse_input(input).finish();
        match r {
            Ok((input, r)) => {
                assert_eq!("", input);
                Ok(r)
            }
            Err(e) => {
                let error = convert_error(input, e);
                println!("{}", &error);
                Err(anyhow::anyhow!(error))
            }
        }
    }

    fn part1((array, folds): &Self::Input) -> Result<usize> {
        let array = fold_grid(array, folds[0]);
        Ok(array.into_iter().filter(|&v| v).count())
    }

    fn part2(input: &Self::Input) -> Result<&'static str> {
        let (mut array, folds) = input.clone();
        folds
            .iter()
            .for_each(|fold| array = fold_grid(&array, *fold));

        if cfg!(not(feature = "disable_for_tests")) {
            print_grid(&array);
        }

        Ok("l337")
    }
}

fn print_grid(array: &Array2<bool>) {
    for row in array.axis_iter(Axis(1)) {
        for c in row {
            print!("{}", if *c { '#' } else { '.' });
        }
        println!("");
    }
}

fn fold_grid(array: &Array2<bool>, fold: Fold) -> Array2<bool> {
    let shape = array.shape();
    match fold {
        Fold::H(axis) => Array2::from_shape_fn((shape[0], axis), |(x, y)| {
            array[(x, y)] || array[(x, 2 * axis - y)]
        }),
        Fold::V(axis) => Array2::from_shape_fn((axis, shape[1]), |(x, y)| {
            array[(x, y)] || array[(2 * axis - x, y)]
        }),
    }
}

fn parse_input<'a>(
    input: &'a str,
) -> IResult<&'a str, (Array2<bool>, Vec<Fold>), VerboseError<&str>> {
    let (input, points) = terminated(many1(point), newline)(input)?;
    let (maxx, maxy) = points
        .iter()
        .fold((0, 0), |(x0, y0), &(x, y)| (max(x0, x), max(y0, y)));
    let mut array = Array2::from_shape_fn((maxx + 1, maxy + 1), |_| false);
    points.into_iter().for_each(|point| {
        array[point] = true;
    });
    let (input, folds) = many1(fold)(input)?;
    Ok((input, (array, folds)))
}

fn point(input: &str) -> IResult<&str, (usize, usize), VerboseError<&str>> {
    tuple((
        map(terminated(digit1, tag(",")), |s: &str| {
            parse_int(s.as_bytes())
        }),
        map(terminated(digit1, newline), |s: &str| {
            parse_int(s.as_bytes())
        }),
    ))(input)
}

fn fold(input: &str) -> IResult<&str, Fold, VerboseError<&str>> {
    map(
        delimited(
            tag("fold along "),
            tuple((terminated(one_of("xy"), tag("=")), digit1)),
            newline,
        ),
        |(d, n): (char, &str)| {
            let n = parse_int(n.as_bytes());
            match d {
                'x' => Fold::V(n),
                'y' => Fold::H(n),
                _ => unreachable!(),
            }
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(17, Day::part1(&input)?);
        let (mut array, folds) = input.clone();
        folds
            .iter()
            .for_each(|fold| array = fold_grid(&array, *fold));
        println!("{:?}", array);
        assert_eq!("l337", Day::part2(&input)?);
        Ok(())
    }
}
