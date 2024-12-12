# Advent of Code 2024

## Setup

#### Notes for starting with new computer.

This project uses [git-crypt](https://github.com/AGWA/git-crypt) to encrypt my secrets with gpg.
If I don't have access to the private keys anymore, I can just generate a new `.env` file by 
copying the `dot-env.example` file.

```sh
cp dot-env.example .env
```

## Running days

To run all days:

```sh
cargo run --release
```

Or to run a specific day:
```sh
cargo run --release -- -d 1
```

## Benchmarks

Timings generated with:

The `cargo-criterion` crate is useful to get nice benchmarks.

```sh
cargo criterion
```

Though not required, this just doesn't have as nice output, and will deprecate plots soon:

```sh
cargo bench
```

### Profiling

The `cpuprofiler` and `pprof` crates are used to profile the benchmark code.

Enable profiling with:

```sh
cargo bench --bench criterion -- --profile-time=10
```

## Timings

```
day01/get_input         time:   [52.690 µs 53.011 µs 53.374 µs]
day01/part1             time:   [12.893 µs 13.110 µs 13.386 µs]
day01/part2             time:   [31.882 µs 32.071 µs 32.310 µs]

day02/get_input         time:   [62.530 µs 62.573 µs 62.615 µs]
day02/part1             time:   [4.8615 µs 4.8665 µs 4.8720 µs]
day02/part2             time:   [51.079 µs 51.210 µs 51.342 µs]

day03/get_input         time:   [115.21 µs 115.31 µs 115.43 µs]
day03/part1             time:   [286.51 ns 286.89 ns 287.56 ns]
day03/part2             time:   [384.34 ns 384.55 ns 384.78 ns]

day04/get_input         time:   [35.461 µs 35.500 µs 35.541 µs]
day04/part1             time:   [293.84 µs 293.92 µs 294.02 µs]
day04/part2             time:   [1.1441 ms 1.1454 ms 1.1470 ms]

day05/get_input         time:   [115.96 µs 116.07 µs 116.19 µs]
day05/part1             time:   [8.4516 µs 8.5001 µs 8.5654 µs]
day05/part2             time:   [62.037 µs 62.082 µs 62.128 µs]

day06/get_input         time:   [21.860 µs 21.942 µs 22.046 µs]
day06/part1             time:   [157.81 µs 158.62 µs 159.81 µs]
day06/part2             time:   [160.41 ms 160.72 ms 161.08 ms]

day07/get_input         time:   [232.75 µs 233.04 µs 233.33 µs]
day07/part1             time:   [1.1775 ms 1.1792 ms 1.1812 ms]
day07/part2             time:   [42.226 ms 42.264 ms 42.306 ms]

day08/get_input         time:   [11.912 µs 11.931 µs 11.956 µs]
day08/part1             time:   [10.442 µs 10.458 µs 10.474 µs]
day08/part2             time:   [41.670 µs 41.694 µs 41.721 µs]

day09/get_input         time:   [289.46 µs 290.03 µs 290.83 µs]
day09/part1             time:   [242.38 µs 243.45 µs 244.76 µs]
day09/part2             time:   [55.630 ms 55.660 ms 55.696 ms]

day10/get_input         time:   [9.8250 µs 9.8431 µs 9.8579 µs]
day10/part1             time:   [225.61 µs 225.84 µs 226.14 µs]
day10/part2             time:   [135.68 µs 135.74 µs 135.80 µs]

day11/get_input         time:   [235.03 ns 235.45 ns 236.29 ns]
day11/part1             time:   [132.37 µs 132.52 µs 132.76 µs]
day11/part2             time:   [5.6205 ms 5.6402 ms 5.6679 ms]

day12/get_input         time:   [469.17 µs 470.29 µs 471.67 µs]
day12/part1             time:   [7.4980 ms 7.5315 ms 7.5821 ms]
day12/part2             time:   [8.6344 ms 8.6521 ms 8.6807 ms]

```
<details>
Original timings:

```
day02/get_input         time:   [100.97 µs 101.08 µs 101.21 µs]
day02/part1             time:   [4.4582 µs 4.4625 µs 4.4669 µs]
day02/part2             time:   [60.616 µs 60.749 µs 60.903 µs]

day03/get_input         time:   [66.725 µs 66.804 µs 66.889 µs]
day03/part1             time:   [251.68 ns 253.78 ns 256.20 ns]
day03/part2             time:   [394.21 ns 396.88 ns 400.32 ns]

day04/get_input         time:   [36.074 µs 36.171 µs 36.257 µs]
day04/part1             time:   [930.56 µs 932.72 µs 935.74 µs]
day04/part2             time:   [11.553 ms 11.562 ms 11.572 ms]

day05/get_input         time:   [108.92 µs 110.35 µs 112.14 µs]
day05/part1             time:   [30.334 µs 30.757 µs 31.268 µs]
day05/part2             time:   [327.25 µs 331.89 µs 337.42 µs]

day06/get_input         time:   [31.677 µs 31.741 µs 31.823 µs]
day06/part1             time:   [417.21 µs 417.57 µs 418.12 µs]
day06/part2             time:   [1.3599 s 1.3613 s 1.3628 s]

day07/get_input         time:   [247.12 µs 247.33 µs 247.59 µs]
day07/part1             time:   [20.896 ms 20.912 ms 20.929 ms]
day07/part2             time:   [2.5735 s 2.5755 s 2.5779 s]

day09/get_input         time:   [551.85 µs 552.30 µs 552.83 µs]
day09/part1             time:   [245.84 µs 246.15 µs 246.47 µs]
day09/part2             time:   [1.2063 s 1.2070 s 1.2077 s]

```
</details>
