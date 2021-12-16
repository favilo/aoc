use std::fs::read_to_string;

use criterion::{criterion_group, criterion_main, Criterion};

use aoc2021::Runner;
use pprof::{criterion::Output, flamegraph::Options};

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
    ($day:ident, $($days:ident),+ $(,)?) => {
        days! { $day, $($days),+ }
        criterion_group!(
            name = benches;
            config = custom();
            targets = $day,
                $($days),+

        );

        criterion_main!(benches);
    };
}

benches!(
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15,
);

fn custom() -> Criterion {
    let mut options = Options::default();
    options.flame_chart = true;

    Criterion::default().with_profiler(pprof::criterion::PProfProfiler::new(
        1000,
        Output::Flamegraph(Some(options)),
    ))
}
