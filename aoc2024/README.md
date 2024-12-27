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

day12/get_input         time:   [421.01 µs 421.85 µs 423.13 µs]
day12/part1             time:   [6.3636 ms 6.3699 ms 6.3776 ms]
day12/part2             time:   [7.4957 ms 7.5060 ms 7.5205 ms]

day13/get_input         time:   [40.192 µs 40.287 µs 40.404 µs]
day13/part1             time:   [3.9342 µs 3.9349 µs 3.9357 µs]
day13/part2             time:   [3.5701 µs 3.5720 µs 3.5744 µs]

day14/get_input         time:   [43.105 µs 43.705 µs 44.444 µs]
day14/part1             time:   [3.7996 µs 3.8574 µs 3.9278 µs]
day14/part2             time:   [15.482 ms 15.728 ms 15.989 ms]

day15/get_input         time:   [368.81 µs 369.06 µs 369.39 µs]
day15/part1             time:   [6.8635 ms 6.8678 ms 6.8721 ms]
day15/part2             time:   [16.778 ms 16.802 ms 16.830 ms]

day06/get_input         time:   [22.093 µs 22.556 µs 23.130 µs]
day06/part1             time:   [162.24 µs 164.51 µs 167.22 µs]
day06/part2             time:   [28.007 ms 28.357 ms 28.736 ms]

day17/get_input         time:   [322.97 ns 323.47 ns 324.00 ns]
day17/part1             time:   [580.99 ns 581.84 ns 582.75 ns]
day17/part2             time:   [89.339 µs 89.384 µs 89.433 µs]

day18/get_input         time:   [139.98 µs 140.15 µs 140.32 µs]
day18/part1             time:   [349.16 µs 349.28 µs 349.40 µs]
day18/part2             time:   [9.2272 ms 9.2365 ms 9.2462 ms]

day19/get_input         time:   [491.36 µs 515.79 µs 539.79 µs]
day19/part1             time:   [53.165 ms 53.194 ms 53.227 ms]
day19/part2             time:   [54.904 ms 54.963 ms 55.021 ms]

day20/get_input         time:   [532.51 µs 547.30 µs 562.81 µs]
day20/part1             time:   [193.37 ms 195.07 ms 197.03 ms]
day20/part2             time:   [201.46 ms 203.64 ms 205.97 ms]

day21/get_input         time:   [173.97 ns 174.07 ns 174.20 ns]
day21/part1             time:   [132.04 µs 132.19 µs 132.35 µs]
day21/part2             time:   [1.1048 ms 1.1059 ms 1.1072 ms]

day22/get_input         time:   [43.608 µs 43.652 µs 43.708 µs]
day22/part1             time:   [9.4435 ms 9.4446 ms 9.4462 ms]
day22/part2             time:   [79.802 ms 79.883 ms 79.983 ms]

day23/get_input         time:   [618.18 µs 618.49 µs 618.80 µs]
day23/part1             time:   [113.76 µs 114.00 µs 114.41 µs]
day23/part2             time:   [7.2462 ms 7.2634 ms 7.2804 ms]

day24/get_input         time:   [20.993 µs 21.030 µs 21.078 µs]
day24/part1             time:   [14.379 µs 14.395 µs 14.414 µs]
day24/part2             time:   [135.92 µs 135.97 µs 136.01 µs]

day25/get_input         time:   [144.11 µs 144.50 µs 145.01 µs]
day25/part1             time:   [485.25 µs 485.45 µs 485.64 µs]
day25/part2             time:   [1.3148 ns 1.3211 ns 1.3277 ns]
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

day12/get_input         time:   [469.17 µs 470.29 µs 471.67 µs]
day12/part1             time:   [7.4980 ms 7.5315 ms 7.5821 ms]
day12/part2             time:   [8.6344 ms 8.6521 ms 8.6807 ms]

day14/get_input         time:   [42.556 µs 42.853 µs 43.237 µs]
day14/part1             time:   [3.5187 µs 3.5613 µs 3.6284 µs]
day14/part2             time:   [83.164 ms 83.296 ms 83.493 ms]

day16/get_input         time:   [596.75 µs 599.88 µs 603.15 µs]
day16/part1             time:   [6.0181 ms 6.0369 ms 6.0621 ms]
day16/part2             time:   [12.648 ms 12.688 ms 12.732 ms]

```
</details>
