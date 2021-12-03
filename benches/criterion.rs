use std::fs::read_to_string;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc2021::{
    day1,
    day2,
    day3,
    // day4, day5, day6, day7,
    // day8, day9
    // day10, day11, day12, day13, day14, day15, day16,
    Runner,
};

// macro_rules! days {
//     ($day:expr) => {
//         fn day
//     }
// }

fn day01(c: &mut Criterion) {
    let mut group = c.benchmark_group("day01");
    let input = read_to_string(format!("input/2021/day{:02}.txt", day1::Day01::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day1::Day01 as Runner>::get_input(&input))
    });
    let input = <day1::Day01 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day1::Day01 as Runner>::part1(&input))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day1::Day01 as Runner>::part2(&input))
    });
    group.finish();
}

fn day02(c: &mut Criterion) {
    let mut group = c.benchmark_group("day02");
    let input = read_to_string(format!("input/2021/day{:02}.txt", day2::Day02::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day2::Day02 as Runner>::get_input(black_box(&input)))
    });
    let input = <day2::Day02 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day2::Day02 as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day2::Day02 as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

fn day03(c: &mut Criterion) {
    let mut group = c.benchmark_group("day03");
    let input = read_to_string(format!("input/2021/day{:02}.txt", day3::Day03::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day3::Day03 as Runner>::get_input(black_box(&input)))
    });
    let input = <day3::Day03 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day3::Day03 as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day3::Day03 as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

criterion_group!(
    benches,
    day01,
    day02,
    day03,
    // day04,
    // day05,
    // day06,
    // day07,
    // day08,
    // day09,
    // day10,
    // day11,
    // day11_unsafe,
    // day12,
    // day13,
    // day14,
    // day15,
    // day16,
);
criterion_main!(benches);
