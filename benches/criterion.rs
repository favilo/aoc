use std::fs::read_to_string;

use criterion::{criterion_group, criterion_main, Criterion};

use aoc2021::Runner;

macro_rules! days {
    ($day:ident) => {
        use aoc2021::$day;

        fn $day(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($day));
            let input =
                read_to_string(format!("input/2021/day{:02}.txt", $day::Day::day())).unwrap();
            group.bench_function("get_input", |b| {
                b.iter(|| <$day::Day as Runner>::get_input(&input))
            });
            let input = <$day::Day as Runner>::get_input(&input).unwrap();
            group.bench_function("part1", |b| b.iter(|| <$day::Day as Runner>::part1(&input)));
            group.bench_function("part2", |b| b.iter(|| <$day::Day as Runner>::part2(&input)));
            group.finish();
        }
    };
    ($day:ident, $($days:ident),+) => {
        days! { $day }
        days! { $($days),+ }
    };
}

macro_rules! benches {
    ($day:ident, $($days:ident),+) => {
        days! { $day, $($days),+ }
        criterion_group!(
            benches,
            $day,
            $($days),+
        );

        criterion_main!(benches);
    };
}

benches! { day01, day02, day03, day04, day05, day06 }
