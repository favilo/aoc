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


```
<details>
Original timings:

```

```
</details>
