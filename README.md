# Advent of Code 2021

To run all days:

```sh
cargo run --release
```

Timings generated by:

The `cargo-criterion` crate is useful to get nice benchmarks.

```sh
cargo criterion
```

Though not required, this just doesn't have as nice output, and will deprecate plots soon:

```sh
cargo bench
```

## Timings

```
day01/get_input         time:   [45.686 us 45.940 us 46.212 us]
day01/part1             time:   [1.1418 us 1.1446 us 1.1476 us]
day01/part2             time:   [2.7954 us 2.8091 us 2.8235 us]

day02/get_input         time:   [56.037 us 56.204 us 56.364 us]
day02/part1             time:   [924.83 ns 931.67 ns 939.51 ns]
day02/part2             time:   [782.38 ns 785.07 ns 787.92 ns]

day03/get_input         time:   [31.327 us 31.367 us 31.414 us]
day03/part1             time:   [1.9537 us 1.9623 us 1.9725 us]
day03/part2             time:   [12.419 us 12.470 us 12.524 us]
```
