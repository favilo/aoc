use std::collections::HashSet;

use anyhow::Result;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, multispace0, newline},
    combinator::{map, opt},
    error::{context, VerboseError},
    multi::{many1, many_m_n},
    sequence::{delimited, terminated},
    Finish, IResult,
};

use crate::Runner;

pub struct Day;

type Coord = (usize, usize);

#[derive(Debug, Clone)]
struct Board {
    // grid: Array2<usize>,
    grid: [(bool, Option<Coord>); 100],
    rows: [usize; 5],
    cols: [usize; 5],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            grid: [(false, None); 100],
            rows: Default::default(),
            cols: Default::default(),
        }
    }
}

impl Board {
    fn mark_number(&mut self, n: usize) {
        if let (m, Some((x, y))) = self.grid[n] {
            self.grid[n].0 = true;
            if !m {
            self.cols[x] += 1;
            self.rows[y] += 1;
            }
        }
    }

    fn is_solved(&self) -> bool {
        self.rows.iter().any(|&row| row == 5) || self.cols.iter().any(|&col| col == 5)
    }

    fn score(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .filter(|(_, (marked, p))| !marked & p.is_some())
            .fold(0, |a, (idx, _)| a + idx)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Bingo {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

impl Runner for Day {
    type Input = Bingo;
    type Output = usize;

    fn day() -> usize {
        4
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        let (input, bingo) = parse_bingo(input).finish().unwrap();
        assert_eq!("", input);
        Ok(bingo)
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        let input = input.clone();
        let numbers = input.numbers;
        let mut boards = input.boards;
        let (last_num, board) = numbers
            .iter()
            .copied()
            .find_map(|n| {
                boards.iter_mut().for_each(|b| b.mark_number(n));
                if let Some(board) = boards.iter().find(|b| b.is_solved()) {
                    return Some((n, board.clone()));
                }
                None
            })
            .unwrap();
        Ok(last_num * board.score())
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let input = input.clone();
        let numbers = input.numbers;
        let mut boards = input.boards;
        let mut solved = HashSet::new();
        let (last_num, board) = numbers
            .iter()
            .copied()
            .find_map(|n| {
                boards.iter_mut().for_each(|b| b.mark_number(n));
                let now_solved = boards.iter().enumerate().filter(|(_, b)| b.is_solved());
                for (idx, b) in now_solved {
                    if !solved.contains(&idx) {
                        solved.insert(idx);
                        if solved.len() == boards.len() {
                            return Some((n, b.clone()));
                        }
                    }
                }
                None
            })
            .unwrap();
        Ok(last_num * board.score())
    }
}

fn parse_bingo<'a>(input: &'a str) -> IResult<&'a str, Bingo, VerboseError<&'a str>> {
    let (input, line) = terminated(take_until("\n"), many1(newline))(input)?;
    let (line, numbers): (&str, Vec<usize>) = map(many1(terminated(digit1, opt(tag(",")))), |v| {
        v.into_iter()
            .map(|s| usize::from_str_radix(s, 10).unwrap())
            .collect()
    })(line)?;
    assert_eq!("", line);
    let (input, boards) = parse_boards(input)?;
    Ok((input, Bingo { numbers, boards }))
}

fn parse_boards(input: &str) -> IResult<&str, Vec<Board>, VerboseError<&str>> {
    many1(context("board", parse_board))(input)
}

fn parse_board(input: &str) -> IResult<&str, Board, VerboseError<&str>> {
    let (input, rows): (&str, Vec<Vec<usize>>) = many_m_n(
        5,
        5,
        many_m_n(
            5,
            5,
            delimited(
                multispace0,
                map(digit1, |s| usize::from_str_radix(s, 10).unwrap()),
                multispace0,
            ),
        ),
    )(input)?;
    let mut grid = [(false, None); 100];
    rows.into_iter().enumerate().for_each(|(x, cols)| {
        cols.into_iter().enumerate().for_each(|(y, val)| {
            grid[val].1 = Some((x, y));
        })
    });

    Ok((
        input,
        Board {
            grid,
            ..Default::default()
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(4512, Day::part1(&input)?);
        assert_eq!(1924, Day::part2(&input)?);
        Ok(())
    }
}
