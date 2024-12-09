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

day07/get_input         time:   [236.83 µs 237.10 µs 237.39 µs]
day07/part1             time:   [1.2572 ms 1.2584 ms 1.2600 ms]
day07/part2             time:   [251.43 ms 251.70 ms 252.00 ms]

day08/get_input         time:   [11.912 µs 11.931 µs 11.956 µs]
day08/part1             time:   [10.442 µs 10.458 µs 10.474 µs]
day08/part2             time:   [41.670 µs 41.694 µs 41.721 µs]

day09/get_input         time:   [551.85 µs 552.30 µs 552.83 µs]
day09/part1             time:   [245.84 µs 246.15 µs 246.47 µs]
day09/part2             time:   [1.2063 s 1.2070 s 1.2077 s]

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

```
</details>
