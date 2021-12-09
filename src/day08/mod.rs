use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0},
    combinator::map,
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};

use crate::Runner;

pub struct Day;

fn bitset(input: &[u8]) -> IResult<&[u8], u8> {
    map(terminated(alpha1, multispace0), |s: &[u8]| {
        s.iter().fold(0, |a, b| a | (1 << (b - 'a' as u8)))
    })(input)
}

type Updated = bool;

const ZERO: usize = 0b111_0111;
const ONE: usize = 0b010_0100;
const TWO: usize = 0b101_1101;
const THREE: usize = 0b110_1101;
const FOUR: usize = 0b010_1110;
const FIVE: usize = 0b110_1011;
const SIX: usize = 0b111_1011;
const SEVEN: usize = 0b010_0101;
const EIGHT: usize = 0b111_1111;
const NINE: usize = 0b110_1111;

// Index = segment section
// Value = possible segments
#[derive(Clone)]
struct BitSet([u8; 7]);

#[inline]
fn not(set: u8) -> u8 {
    !set & 0x7f
}

#[inline]
fn first_bit(set: u8) -> Option<usize> {
    bits(set).next()
}

#[inline]
fn bits(set: u8) -> impl Iterator<Item = usize> {
    (0..7).filter(move |b| set >> b & 1 == 1)
}

fn to_letter(bit: usize) -> char {
    ('a' as usize + bit) as u8 as char
}

fn to_letters(byte: usize) -> String {
    bits(byte as u8)
        .into_iter()
        .map(to_letter)
        .collect::<String>()
}

fn decode_segments(keys: &[u8], values: &[u8]) -> usize {
    let mut bitset = BitSet::default();
    let mut fives = 0x7f;
    let mut sixes = 0x7f;
    for &segments in keys {
        match segments.count_ones() {
            2 => bitset.add_data(1, segments),
            3 => bitset.add_data(7, segments),
            4 => bitset.add_data(4, segments),
            5 => {
                fives &= segments;
            }
            6 => {
                sixes &= segments;
            }
            7 => bitset.add_data(8, segments), // Useless
            _ => continue,
        }
    }
    bitset.add_data(2, fives);
    bitset.add_data(3, fives);
    bitset.add_data(5, fives);
    bitset.add_data(0, sixes);
    bitset.add_data(6, sixes);
    bitset.add_data(9, sixes);
    if !bitset.solved() {
        panic!("Not solved")
    }
    values
        .iter()
        .map(|&v| {
            let digit = bitset.decode_digit(v);
            digit
        })
        .fold(0, |a, v| a * 10 + v)
}

impl BitSet {
    fn decode_digit(&self, v: u8) -> usize {
        let pattern = bits(v)
            .map(|bit| {
                self.0
                    .iter()
                    .copied()
                    .enumerate()
                    .find(|&(_, v)| v == 1 << bit)
                    .unwrap()
            })
            .fold(0, |a, bit| a | (1 << bit.0));
        match pattern {
            ZERO => 0,
            ONE => 1,
            TWO => 2,
            THREE => 3,
            FOUR => 4,
            FIVE => 5,
            SIX => 6,
            SEVEN => 7,
            EIGHT => 8,
            NINE => 9,
            _ => unreachable!(),
        }
    }

    fn solved(&self) -> bool {
        (0..7).all(|i| self.segment_solved(i))
    }

    fn add_data(&mut self, digit: u8, segments: u8) {
        let updated = match digit {
            0 | 6 | 9 => [
                self.match_segment_to_possible(0, segments),
                self.match_segment_to_possible(1, segments),
                self.match_segment_to_possible(5, segments),
                self.match_segment_to_possible(6, segments),
            ]
            .iter()
            .fold(false, |a, b| a || *b),
            1 => {
                // We KNOW this is a 1
                [
                    self.match_segment_to_possible(0, not(segments)),
                    self.match_segment_to_possible(1, not(segments)),
                    self.match_segment_to_possible(2, segments),
                    self.match_segment_to_possible(3, not(segments)),
                    self.match_segment_to_possible(4, not(segments)),
                    self.match_segment_to_possible(5, segments),
                    self.match_segment_to_possible(6, not(segments)),
                ]
                .iter()
                .fold(false, |a, b| a || *b)
            }
            2 | 3 | 5 => [
                self.match_segment_to_possible(0, segments),
                self.match_segment_to_possible(3, segments),
                self.match_segment_to_possible(6, segments),
            ]
            .iter()
            .fold(false, |a, b| a || *b),
            4 => {
                // We KNOW this is a 4
                [
                    self.match_segment_to_possible(0, not(segments)),
                    self.match_segment_to_possible(1, segments),
                    self.match_segment_to_possible(2, segments),
                    self.match_segment_to_possible(3, segments),
                    self.match_segment_to_possible(4, not(segments)),
                    self.match_segment_to_possible(5, segments),
                    self.match_segment_to_possible(6, not(segments)),
                ]
                .iter()
                .fold(false, |a, b| a || *b)
            }
            7 => {
                // We KNOW this is a 7
                [
                    self.match_segment_to_possible(0, segments),
                    self.match_segment_to_possible(1, not(segments)),
                    self.match_segment_to_possible(2, segments),
                    self.match_segment_to_possible(3, not(segments)),
                    self.match_segment_to_possible(4, not(segments)),
                    self.match_segment_to_possible(5, segments),
                    self.match_segment_to_possible(6, not(segments)),
                ]
                .iter()
                .fold(false, |a, b| a || *b)
            }
            8 => false,
            _ => unreachable!(),
        };
        if updated {
            for seg in 0..7 {
                let possible = self.0[seg];
                if possible.count_ones() == 1 {
                    (0..seg).chain(seg + 1..7).for_each(|s| {
                        self.match_segment_to_possible(s, not(possible));
                    });
                }
            }
        }
    }

    fn segment_solved(&self, seg: usize) -> bool {
        self.0[seg].count_ones() == 1
    }

    fn match_segment_to_possible(&mut self, seg: usize, possible: u8) -> Updated {
        if self.segment_solved(seg) {
            return false;
        }
        let old = self.0.get_mut(seg).unwrap();
        if *old == possible {
            false
        } else {
            *old &= possible;
            assert_ne!(*old, 0);
            true
        }
    }
}

impl Default for BitSet {
    fn default() -> Self {
        Self([0b0111_1111; 7])
    }
}

impl std::fmt::Debug for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("BitSet");
        self.0.iter().enumerate().fold(&mut s, |s, (i, v)| {
            if v.count_ones() == 1 {
                s.field(
                    &format!("{}", to_letter(i)),
                    &format!("{}", to_letter(first_bit(*v).unwrap())),
                )
            } else if *v == 0 {
                s.field(&format!("{}", to_letter(i)), &format!("{}", v))
            } else {
                let bits = to_letters(*v as usize);
                s.field(&format!("{}", to_letter(i)), &format!("{}", bits))
            }
        });

        s.finish()
    }
}

fn parse_line(input: &[u8]) -> IResult<&[u8], (Vec<u8>, Vec<u8>)> {
    let (input, (key, value)) = tuple((
        terminated(many1(bitset), tag(b"| ")),
        terminated(many1(bitset), multispace0),
    ))(input)?;
    assert_eq!(b"", input);
    Ok((input, (key, value)))
}

impl Runner for Day {
    type Input = Vec<(Vec<u8>, Vec<u8>)>;
    type Output = usize;

    fn day() -> usize {
        8
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .lines()
            .map(str::as_bytes)
            .map(parse_line)
            .map(Result::unwrap)
            .map(|t| t.1)
            .collect())
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        Ok(input
            .iter()
            .map(|t| t.1.iter())
            .flatten()
            .filter(|b| match b.count_ones() {
                2 => true, // one
                3 => true, // seven
                4 => true, // four
                7 => true, // eight
                _ => false,
            })
            .count())
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let s: usize = input.into_iter().map(|(k, v)| decode_segments(k, v)).sum();
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(26, Day::part1(&input)?);
        assert_eq!(61229, Day::part2(&input)?);
        Ok(())
    }

    #[test]
    fn digits() -> Result<()> {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let input = Day::get_input(input)?;
        let segments = decode_segments(&input[0].0, &input[0].1);
        // let segments = decode_segments(&input[0].0, &vec![0x03]);
        assert_eq!(5353, segments);
        Ok(())
    }
}
