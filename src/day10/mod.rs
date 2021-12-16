use std::iter::repeat;

use anyhow::Result;

use crate::{utils::median, Runner};

pub struct Day;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    Paren,
    Angle,
    Curly,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Open,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bracket(State, Type);

impl Type {
    fn score(&self) -> usize {
        match self {
            Type::Paren => 3,
            Type::Square => 57,
            Type::Curly => 1197,
            Type::Angle => 25137,
        }
    }

    fn incomplete_score(&self) -> usize {
        match self {
            Type::Paren => 1,
            Type::Square => 2,
            Type::Curly => 3,
            Type::Angle => 4,
        }
    }
}

impl TryFrom<char> for Bracket {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '(' => Bracket(State::Open, Type::Paren),
            ')' => Bracket(State::Closed, Type::Paren),
            '<' => Bracket(State::Open, Type::Angle),
            '>' => Bracket(State::Closed, Type::Angle),
            '[' => Bracket(State::Open, Type::Square),
            ']' => Bracket(State::Closed, Type::Square),
            '{' => Bracket(State::Open, Type::Curly),
            '}' => Bracket(State::Closed, Type::Curly),
            _ => return Err(()),
        })
    }
}

impl Runner for Day {
    type Input = Vec<(Vec<(Type, isize)>, Option<Type>)>;

    fn day() -> usize {
        10
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input.lines().map(pairs).map(Result::unwrap).collect())
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        Ok(input.iter().map(|t| t.1).flatten().map(|b| b.score()).sum())
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        let mut scores = input
            .iter()
            .filter(|t| t.1.is_none())
            .map(|t| &t.0)
            .map(|v| finish(&v))
            .collect::<Vec<_>>();
        scores.sort();
        Ok(median(&scores))
    }
}

fn pairs(input: &str) -> Result<(Vec<(Type, isize)>, Option<Type>)> {
    let v: Result<Vec<(Type, isize)>, Bracket> =
        input.chars().try_fold(Vec::<(Type, isize)>::new(), |v, c| {
            let mut v = v.clone();
            let b = Bracket::try_from(c).unwrap();
            let len = v.len();
            if len == 0 {
                v.push((b.1, 0));
            }
            let last = *v.last().unwrap();
            match b {
                Bracket(State::Open, t) => {
                    if last.0 == t {
                        v.last_mut().unwrap().1 += 1;
                    } else {
                        v.push((t, 1));
                    }
                }
                Bracket(State::Closed, t) => {
                    if last.0 == t {
                        v.last_mut().unwrap().1 -= 1;
                    } else {
                        return Err(b);
                    }
                }
            }
            if v[v.len() - 1].1 == 0 {
                v.pop();
            }
            Ok(v)
        });
    Ok(if let Err(e) = v {
        (Default::default(), Some(e.1))
    } else {
        (v.unwrap(), None)
    })
}

fn finish(input: &[(Type, isize)]) -> usize {
    input
        .iter()
        .rev()
        .map(|(t, c)| repeat(t).take(*c as usize))
        .flatten()
        .fold(0, |a, v| a * 5 + v.incomplete_score())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(26397, Day::part1(&input)?);
        assert_eq!(288957, Day::part2(&input)?);
        Ok(())
    }

    #[test]
    fn simple() -> Result<()> {
        let input = "(((((((((())))))))))
[<>({}){}[([])<>]]";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(0, Day::part1(&input)?);
        Ok(())
    }

    #[test]
    fn corrupted() -> Result<()> {
        let input = "(]";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(57, Day::part1(&input)?);
        Ok(())
    }

    #[test]
    fn incomplete() -> Result<()> {
        let input = "((())[{";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(0, Day::part1(&input)?);
        assert_eq!(17 * 5 + 1, Day::part2(&input)?);
        Ok(())
    }
}
