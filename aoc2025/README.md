# Advent of Code 2025

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

```
<details>
Original timings:

```
day01/get_input         time:   [45.273 µs 45.526 µs 45.782 µs]
day01/part1             time:   [7.5001 µs 7.5304 µs 7.5635 µs]
day01/part2             time:   [17.220 µs 17.264 µs 17.313 µs]

```
</details>
