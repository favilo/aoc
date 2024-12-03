use std::fs::read_to_string;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc2020::{
    day1, day10, day11, day12, day13, day14, day15, day16, day2, day3, day4, day5, day6, day7,
    day8, day9, Runner,
};

fn day01(c: &mut Criterion) {
    let mut group = c.benchmark_group("day01");
    let input = read_to_string(format!("input/2020/day{}.txt", day1::Day01::day())).unwrap();
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
    let input = read_to_string(format!("input/2020/day{}.txt", day2::Day02::day())).unwrap();
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
    let input = read_to_string(format!("input/2020/day{}.txt", day3::Day03::day())).unwrap();
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

fn day04(c: &mut Criterion) {
    let mut group = c.benchmark_group("day04");
    let input = read_to_string(format!("input/2020/day{}.txt", day4::Day04::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day4::Day04 as Runner>::get_input(black_box(&input)))
    });
    let input = <day4::Day04 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day4::Day04 as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day4::Day04 as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

fn day05(c: &mut Criterion) {
    let mut group = c.benchmark_group("day05");
    let input = read_to_string(format!("input/2020/day{}.txt", day5::Day05::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day5::Day05 as Runner>::get_input(black_box(&input)))
    });
    let input = <day5::Day05 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day5::Day05 as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day5::Day05 as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

fn day06(c: &mut Criterion) {
    let mut group = c.benchmark_group("day06");
    let input = read_to_string(format!("input/2020/day{}.txt", day6::Day06::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day6::Day06 as Runner>::get_input(black_box(&input)))
    });
    let input = <day6::Day06 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day6::Day06 as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day6::Day06 as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

fn day07(c: &mut Criterion) {
    let mut group = c.benchmark_group("day07");
    let input = read_to_string(format!("input/2020/day{}.txt", day7::Day07::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day7::Day07 as Runner>::get_input(black_box(&input)))
    });
    let input = <day7::Day07 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day7::Day07 as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day7::Day07 as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

fn day08(c: &mut Criterion) {
    let mut group = c.benchmark_group("day08");
    let input = read_to_string(format!("input/2020/day{}.txt", day8::Day08::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day8::Day08 as Runner>::get_input(black_box(&input)))
    });
    let input = <day8::Day08 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day8::Day08 as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day8::Day08 as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

// TODO: See if I can get rid of this custom crap
fn day09(c: &mut Criterion) {
    let mut group = c.benchmark_group("day09");
    let input = read_to_string(format!("input/2020/day{}.txt", day9::Day09::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day9::Day09 as Runner>::get_input(black_box(&input)))
    });
    let input = <day9::Day09 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| day9::get_oddball(black_box(&input), 26))
    });
    let output = day9::get_oddball(black_box(&input), 26).unwrap();
    group.bench_function("part2", |b| {
        b.iter(|| day9::get_run(black_box(&input), output.0, output.1))
    });
    group.finish();
}

fn day10(c: &mut Criterion) {
    let mut group = c.benchmark_group("day10");
    let input = read_to_string(format!("input/2020/day{}.txt", day10::Day10::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day10::Day10 as Runner>::get_input(black_box(&input)))
    });
    let input = <day10::Day10 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day10::Day10 as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day10::Day10 as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

fn day11(c: &mut Criterion) {
    let mut group = c.benchmark_group("day11");
    let input = read_to_string(format!("input/2020/day{}.txt", day11::Day11::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day11::Day11 as Runner>::get_input(black_box(&input)))
    });
    let input = <day11::Day11 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day11::Day11 as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day11::Day11 as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

fn day11_unsafe(c: &mut Criterion) {
    let mut group = c.benchmark_group("day11 unsafe");
    let input = read_to_string(format!("input/2020/day{}.txt", day11::Day11::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day11::Day11Unsafe as Runner>::get_input(black_box(&input)))
    });
    let input = <day11::Day11Unsafe as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day11::Day11Unsafe as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day11::Day11Unsafe as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

fn day12(c: &mut Criterion) {
    let mut group = c.benchmark_group("day12");
    let input = read_to_string(format!("input/2020/day{}.txt", day12::Day12::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day12::Day12 as Runner>::get_input(black_box(&input)))
    });
    let input = <day12::Day12 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day12::Day12 as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day12::Day12 as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

fn day13(c: &mut Criterion) {
    let mut group = c.benchmark_group("day13");
    let input = read_to_string(format!("input/2020/day{}.txt", day13::Day13::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day13::Day13 as Runner>::get_input(black_box(&input)))
    });
    let input = <day13::Day13 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day13::Day13 as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day13::Day13 as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

fn day14(c: &mut Criterion) {
    let mut group = c.benchmark_group("day14");
    let input = read_to_string(format!("input/2020/day{}.txt", day14::Day14::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day14::Day14 as Runner>::get_input(black_box(&input)))
    });
    let input = <day14::Day14 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day14::Day14 as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day14::Day14 as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

fn day15(c: &mut Criterion) {
    let mut group = c.benchmark_group("day15");
    let input = read_to_string(format!("input/2020/day{}.txt", day15::Day15::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day15::Day15 as Runner>::get_input(black_box(&input)))
    });
    let input = <day15::Day15 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day15::Day15 as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day15::Day15 as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

fn day16(c: &mut Criterion) {
    let mut group = c.benchmark_group("day16");
    let input = read_to_string(format!("input/2020/day{}.txt", day16::Day16::day())).unwrap();
    group.bench_function("get_input", |b| {
        b.iter(|| <day16::Day16 as Runner>::get_input(black_box(&input)))
    });
    let input = <day16::Day16 as Runner>::get_input(&input).unwrap();
    group.bench_function("part1", |b| {
        b.iter(|| <day16::Day16 as Runner>::part1(black_box(&input)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| <day16::Day16 as Runner>::part2(black_box(&input)))
    });
    group.finish();
}

criterion_group!(
    benches,
    day01,
    day02,
    day03,
    day04,
    day05,
    day06,
    day07,
    day08,
    day09,
    day10,
    day11,
    day11_unsafe,
    day12,
    day13,
    day14,
    day15,
    day16,
);
criterion_main!(benches);
