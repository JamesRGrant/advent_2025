# Advent of Code 2025
Solutions in Rust.

See [www.adventofcode.com](https://www.adventofcode.com)

## Input
There are two files for each day.  One contains test data, one contains the user's data.

## Code Structure
`main.rs` loops through the days, parses the data for a day into an array, and passes it to a day solver module, which then does each problem.
Each day is in an individual module.
There is a python script to generate a new module each day

## Linting
`main.rs` is configured to use the strictest Clippy linting settings.

## Testing
`cargo test --release` will run the following tests:
* test data for each problem (2 per day)
* user data for each problem

The correct answers are set in an array in the `[test]` section of each module

## Run
`cargo run --release`

## Performance optimizations
I enjoy making my code as fast as possible without resorting to unreadable code.

These times are running WSL2 in Windows Pro 25H2 on a AMD Ryzen 9 6900HX with 32 GB RAM:
```
01: load/parse      in 418.955µs
 1:            1177 in  15.939µs
 2:            6768 in  26.457µs
02: load/parse      in  42.015µs
 1:     17077011375 in   2.390ms
 2:     36037497037 in  50.000ns
03: load/parse      in  42.696µs
 1:           17155 in  13.163µs
 2: 169685670469164 in  44.318µs
04: load/parse      in 140.670µs
 1:            1363 in  60.000ns
 2:            8184 in 530.774µs
05: load/parse      in 100.479µs
 1:             520 in  58.174µs
 2: 347338785050515 in 100.000ns
06: load/parse      in  94.097µs
 1:   7326876294741 in   4.818µs
 2:  10756006415204 in  73.941µs
Total elapsed time:      4.243ms
```