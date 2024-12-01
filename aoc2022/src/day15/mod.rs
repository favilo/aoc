use std::{
    iter::once,
    ops::{Add, AddAssign, Range},
};

use color_eyre::Result;
use itertools::{interleave, Itertools};
use nom::{
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    combinator::{all_consuming, opt},
    error::VerboseError,
    multi::many1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::{utils::RangeIncExt, Runner};

pub struct Day;

type IsProduction = bool;
type SInput<'a> = &'a str;
type ParseResult<'a, T> = IResult<SInput<'a>, T, VerboseError<SInput<'a>>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(i64, i64);

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Coord {
    fn manhattan_distance(self, other: Self) -> i64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SensorPair {
    sensor: Coord,
    beacon: Coord,
    distance: i64,
}

impl SensorPair {
    fn new(sensor: Coord, beacon: Coord) -> Self {
        Self {
            sensor,
            beacon,
            distance: sensor.manhattan_distance(beacon),
        }
    }

    fn manhattan_distance(self) -> i64 {
        self.distance
    }

    fn contains(self, coord: Coord) -> bool {
        self.sensor.manhattan_distance(coord) <= self.manhattan_distance()
    }

    fn perimeter(self) -> impl Iterator<Item = Coord> {
        let SensorPair {
            sensor, distance, ..
        } = self;
        let distance = distance + 1;
        (0..distance).flat_map(move |d| {
            [
                sensor + Coord(d, d - distance),
                sensor + Coord(distance - d, d),
                sensor + Coord(-d, distance - d),
                sensor + Coord(d - distance, -d),
            ]
        })
    }
}

fn coord(input: SInput<'_>) -> ParseResult<'_, Coord> {
    let (input, (x, y)) = tuple((preceded(tag("x="), i64), preceded(tag(", y="), i64)))(input)?;
    Ok((input, Coord(x, y)))
}

fn sensor_pair(input: SInput<'_>) -> ParseResult<'_, SensorPair> {
    let (input, (sensor, beacon)) = terminated(
        tuple((
            preceded(tag("Sensor at "), coord),
            preceded(tag(": closest beacon is at "), coord),
        )),
        opt(line_ending),
    )(input)?;
    Ok((input, SensorPair::new(sensor, beacon)))
}

fn union_ranges(ranges: &[Range<i64>]) -> Vec<Range<i64>> {
    ranges
        .iter()
        .fold(Vec::new(), |acc, r| add_range_to_union(r.clone(), &acc))
}

fn add_range_to_union(range: Range<i64>, union: &[Range<i64>]) -> Vec<Range<i64>> {
    if range.is_empty() {
        return union.to_vec();
    }
    union.iter().fold(vec![range], |acc, range| {
        let larger = acc
            .iter()
            .filter(|r| range.overlaps(r))
            .fold(range.clone(), |acc, r| r.union(&acc).unwrap());
        once(larger)
            .chain(acc.into_iter().filter(|r| !range.overlaps(r)))
            .collect_vec()
    })
}

fn count_no_beacon_in_row(row: i64, pairs: &[SensorPair], limits: Range<i64>) -> usize {
    let ranges = ranges_for_row(row, pairs, limits);
    ranges.iter().map(|r| r.end - r.start).sum::<i64>() as usize
}

fn ranges_for_row(row: i64, pairs: &[SensorPair], limits: Range<i64>) -> Vec<Range<i64>> {
    let ranges = pairs
        .iter()
        .map(|s @ SensorPair { sensor, .. }| (sensor, s.manhattan_distance()))
        .filter(|(sensor, distance)| (sensor.1 - row).abs() <= *distance)
        .map(|(sensor, distance)| {
            let h = (sensor.1 - row).abs();
            let num_seen = distance - h;
            (sensor.0 - num_seen)..(sensor.0 + num_seen)
        })
        .map(|s| s.intersect(&limits))
        .collect_vec();
    let ranges = union_ranges(&ranges);
    if ranges.len() == 2
        && (ranges[0].start == ranges[1].end + 1 || ranges[1].start == ranges[0].end + 1)
    {
        return vec![ranges[0].start.min(ranges[1].start)..ranges[0].end.max(ranges[1].end)];
    }
    ranges
}

#[allow(dead_code)]
fn get_freq_cheat(limit: i64, pairs: &[SensorPair], estimate: f32) -> usize {
    // Middle out
    let row = interleave(
        (limit as f32 * estimate) as i64..=limit,
        (0..(limit as f32 * estimate) as i64).rev(),
    )
    .map(|row| (row, ranges_for_row(row, pairs, 0..limit)))
    .inspect(|r| {
        if limit <= 20 {
            eprintln!("{r:?}")
        }
    })
    .find(|(_, ranges)| ranges.len() > 1)
    .unwrap();
    let ranges = row.1;
    let row = row.0;
    let col = ranges[0].end.min(ranges[1].end) + 1;
    (col * 4_000_000 + row) as usize
}

fn get_freq_perimeter(limit: i64, pairs: &[SensorPair]) -> usize {
    let Coord(col, row) = pairs
        .iter()
        .flat_map(|p| p.perimeter())
        .filter(|c| [c.0, c.1].into_iter().all(|c| (0..=limit).contains(&c)))
        .find(|c| pairs.iter().all(|p| !p.contains(*c)))
        .unwrap();

    // println!("{row} x {col}");
    (col * 4_000_000 + row) as usize
}

impl Runner for Day {
    type Input<'input> = (IsProduction, Vec<SensorPair>);

    fn day() -> usize {
        15
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        let vec = all_consuming(many1(sensor_pair))(input).unwrap().1;
        // vec.sort_by_key(|p| p.sensor.0);
        Ok((vec.len() != 14, vec))
    }

    fn part1(input: &Self::Input<'_>) -> Result<usize> {
        let row = if input.0 { 2000000 } else { 10 };
        Ok(count_no_beacon_in_row(row, &input.1, -row * 4..row * 4))
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let prod = input.0;
        let limit = if prod { 4000000 } else { 20 };
        if cfg!(test) {
            Ok(get_freq_cheat(limit, &input.1, 0.85))
        } else {
            Ok(get_freq_perimeter(limit, &input.1))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
                Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
                Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
                Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
                Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
                Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
                Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
                Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
                Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
                Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
                Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
                Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
                Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
                Sensor at x=20, y=1: closest beacon is at x=15, y=3\n";
            part1 = 26;
            part2 = 56000011;
    }

    prod_case! {
        part1 = 5511201;
        part2 = 11318723411840;
    }
    #[test]
    fn union_ranges_works() {
        assert_eq!(union_ranges(&[0..1, 1..2]), vec![0..2]);
        assert_eq!(union_ranges(&[0..1, 2..3]), vec![0..1, 2..3]);
        assert_eq!(union_ranges(&[0..1, 1..2, 2..3]), vec![0..3]);
        assert_eq!(union_ranges(&[0..1, 2..3, 1..2,]), vec![0..3]);
        assert_eq!(union_ranges(&[-2..2, 2..14]), vec![-2..14]);
    }
}
