use std::fs::read_to_string;

use cpuprofiler::PROFILER;
use criterion::profiler::Profiler;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pprof::{criterion::Output, flamegraph::Options};

use aoc_utils::Runner;

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
fn custom() -> Criterion {
    let mut options = Options::default();
    options.flame_chart = true;
    // options.reverse_stack_order = true;
    options.color_diffusion = true;

    Criterion::default().with_profiler(MyProfiler::new(pprof::criterion::PProfProfiler::new(
        1000,
        // Output::Protobuf,
        Output::Flamegraph(Some(options)),
    )))
}

struct MyProfiler<'a, 'b> {
    pprof: pprof::criterion::PProfProfiler<'a, 'b>,
}

impl<'a, 'b> MyProfiler<'a, 'b> {
    fn new(pprof: pprof::criterion::PProfProfiler<'a, 'b>) -> Self {
        Self { pprof }
    }
}

impl Profiler for MyProfiler<'_, '_> {
    fn start_profiling(&mut self, benchmark_id: &str, benchmark_dir: &std::path::Path) {
        use std::fs::create_dir_all;
        self.pprof.start_profiling(benchmark_id, benchmark_dir);
        let fname = benchmark_dir.join("benchmark.profile");
        create_dir_all(fname.parent().unwrap()).unwrap();
        println!("\nStarting profiling to {}", fname.display());
        PROFILER
            .lock()
            .unwrap()
            .start(fname.to_str().unwrap())
            .unwrap();
    }

    fn stop_profiling(&mut self, benchmark_id: &str, benchmark_dir: &std::path::Path) {
        self.pprof.stop_profiling(benchmark_id, benchmark_dir);
        PROFILER.lock().unwrap().stop().unwrap();
    }
}

benches!(
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25,
);
