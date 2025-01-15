# Advent of Code 2019

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
day01/get_input         time:   [1.7799 µs 1.7826 µs 1.7854 µs]
day01/part1             time:   [47.032 ns 47.067 ns 47.121 ns]
day01/part2             time:   [588.45 ns 588.54 ns 588.63 ns]

day02/get_input         time:   [2.8472 µs 2.8613 µs 2.8793 µs]
day02/part1             time:   [253.08 ns 253.27 ns 253.47 ns]
day02/part2             time:   [778.47 ns 779.63 ns 781.42 ns]

day03/get_input         time:   [21.428 µs 21.438 µs 21.452 µs]
day03/part1             time:   [176.17 µs 176.36 µs 176.59 µs]
day03/part2             time:   [180.02 µs 185.83 µs 192.35 µs]

day04/get_input         time:   [22.278 ns 22.535 ns 22.856 ns]
day04/part1             time:   [159.65 µs 160.05 µs 160.56 µs]
day04/part2             time:   [345.61 µs 345.69 µs 345.78 µs]

day05/get_input         time:   [15.404 µs 15.512 µs 15.646 µs]
day05/part1             time:   [3.5775 µs 3.5985 µs 3.6222 µs]
day05/part2             time:   [4.1283 µs 4.1545 µs 4.1839 µs]

```
<details>
Original timings:

```

```
</details>
