use std::fs::read_to_string;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc2024::Runner;
use pprof::{criterion::Output, flamegraph::Options};

macro_rules! days {
    () => {};
    ($day:ident) => {
        use aoc2024::$day;

        fn $day(c: &mut Criterion) {
            use aoc_utils::utils::file::get_input_path;
            let mut group = c.benchmark_group(stringify!($day));
            let input_path = get_input_path(2024, $day::Day::day()).unwrap();
            let input =
                read_to_string(input_path).unwrap();
            group.bench_function("get_input", |b| {
                b.iter(|| black_box($day::Day::get_input(&input)))
            });
            let input = $day::Day::get_input(&input).unwrap();
            group.bench_function("part1", |b| b.iter(|| black_box($day::Day::part1(&input))));
            group.bench_function("part2", |b| b.iter(|| black_box($day::Day::part2(&input))));
            group.finish();
        }
    };
    ($day:ident, $($days:ident),*) => {
        days! { $day }
        days! { $($days),* }
    };
}

macro_rules! benches {
    ($day:ident, $($days:ident),* $(,)?) => {
        days! { $day, $($days),* }
        criterion_group!(
            name = benches;
            config = custom();
            targets = $day,
                $($days),*
        );

        criterion_main!(benches);
    };
}

benches!(day01, day02, day03,);

fn custom() -> Criterion {
    let mut options = Options::default();
    options.flame_chart = true;

    Criterion::default().with_profiler(pprof::criterion::PProfProfiler::new(
        1000,
        Output::Flamegraph(Some(options)),
    ))
}
