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

day04/get_input         time:   [35.918 µs 36.072 µs 36.212 µs]
day04/part1             time:   [225.63 µs 225.88 µs 226.19 µs]
day04/part2             time:   [10.010 ms 10.046 ms 10.091 ms]

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

```
</details>
