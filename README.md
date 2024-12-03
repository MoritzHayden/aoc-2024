# 🎄 Advent of Code 2024

Solutions for [Advent of Code 2024](https://adventofcode.com/2024) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2024 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2024/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2024/day/2) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 1](./src/bin/01.rs) | `293.4µs` | `319.4µs` |
| [Day 2](./src/bin/02.rs) | `115.0µs` | `161.0µs` |
| [Day 3](./src/bin/03.rs) | `-` | `-` |

**Total: 0.89ms**
<!--- benchmarking table --->

---

## Setup

1. Install the [Rust toolchain](https://www.rust-lang.org/tools/install)
2. Install [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli/) via Cargo: `cargo install aoc-cli`
3. Create the file `<home_directory>/.adventofcode.session` and paste your AoC session cookie into it

---

## Usage

### Scaffold Solution

```sh
# example: `cargo scaffold 1`
cargo scaffold <day>

# output:
# Created module file "src/bin/01.rs"
# Created empty input file "data/inputs/01.txt"
# Created empty example file "data/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

### Download Input

```sh
# example: `cargo download 1`
# alternative: `cargo scaffold 1 --download`
cargo download <day>

# output:
# [INFO  aoc] 🎄 aoc-cli - Advent of Code command-line tool
# [INFO  aoc_client] 🎅 Saved puzzle to 'data/puzzles/01.md'
# [INFO  aoc_client] 🎅 Saved input to 'data/inputs/01.txt'
# ---
# 🎄 Successfully wrote input to "data/inputs/01.txt".
# 🎄 Successfully wrote puzzle to "data/puzzles/01.md".
```

### Run Solution

```sh
# example: `cargo solve 01`
# alternative: `cargo solve 01 --release`
cargo solve <day>

# output:
#     Finished dev [unoptimized + debuginfo] target(s) in 0.13s
#     Running `target/debug/01`
# Part 1: 42 (166.0ns)
# Part 2: 42 (41.0ns)
```

### Submit Solution

```sh
# example: `cargo solve 01 --submit 1`
cargo solve <day> --submit <part>
```

### Run All Solutions

```sh
# alternative: `cargo all --release`
cargo all

# output:
#     Running `target/release/advent_of_code`
# ----------
# | Day 01 |
# ----------
# Part 1: 42 (19.0ns)
# Part 2: 42 (19.0ns)
# <...other days...>
# Total: 0.20ms
```

### Benchmark Solutions

```sh
# example: `cargo time 8 --store`
cargo time <day> [--all] [--store]

# output:
# Day 08
# ------
# Part 1: 1 (39.0ns @ 10000 samples)
# Part 2: 2 (39.0ns @ 10000 samples)
#
# Total (Run): 0.00ms
#
# Stored updated benchmarks.
```

### Read Puzzle Description

```sh
# example: `cargo read 1`
cargo read <day>

# output:
# Loaded session cookie from "/Users/<snip>/.adventofcode.session".
# Fetching puzzle for day 1, 2022...
# ...the input...
```

### Scaffold, Download, and Read Current Day

```sh
# example: `cargo today` on December 1st
cargo today

# output:
# Created module file "src/bin/01.rs"
# Created empty input file "data/inputs/01.txt"
# Created empty example file "data/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
# [INFO  aoc] 🎄 aoc-cli - Advent of Code command-line tool
# [INFO  aoc_client] 🎅 Saved puzzle to 'data/puzzles/01.md'
# [INFO  aoc_client] 🎅 Saved input to 'data/inputs/01.txt'
# ---
# 🎄 Successfully wrote input to "data/inputs/01.txt".
# 🎄 Successfully wrote puzzle to "data/puzzles/01.md".
#
# Loaded session cookie from "/Users/<snip>/.adventofcode.session".
# Fetching puzzle for day 1, 2022...
# ...the input...
```

### Run Tests

```sh
cargo test
```

### Run Formatter

```sh
cargo fmt
```

### Run Linter

```sh
cargo clippy
```

### Run Profiler

```sh
# example: `cargo solve 1 --dhat`
cargo solve <day> --dhat
```
