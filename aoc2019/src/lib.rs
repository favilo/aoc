#![feature(associated_type_defaults)]
#![feature(binary_heap_into_iter_sorted)]
#![feature(debug_closure_helpers)]
#![warn(clippy::all)]
//#![warn(clippy::pedantic)]

use aoc_utils::run_days;

// Parse this to prevent formatting from ruining template
pub const YEAR: usize = {
    match usize::from_str_radix("2019", 10) {
        Ok(year) => year,
        _ => panic!("2019 is not a valid number"),
    }
};

mod intcode;

run_days!(
    day01 = 1,
    day02 = 2,
    // day03 = 3,
    // day04 = 4,
    // day05 = 5,
    // day06 = 6,
    // day07 = 7,
    // day08 = 8,
    // day09 = 9,
    // day10 = 10,
    // day11 = 11,
    // day12 = 12,
    // day13 = 13,
    // day14 = 14,
    // day15 = 15,
    // day16 = 16,
    // day17 = 17,
    // day18 = 18,
    // day19 = 19,
    // day20 = 20,
    // day21 = 21,
    // day22 = 22,
    // day23 = 23,
    // day24 = 24,
    // day25 = 25,
);
