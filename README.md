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
01: load/parse      in 431.867µs
 1:            1177 in  16.008µs
 2:            6768 in  25.132µs
02: load/parse      in  39.359µs
 1:     17077011375 in   2.357ms
 2:     36037497037 in  60.000ns
03: load/parse      in  29.885µs
 1:           17155 in  61.340µs
 2: 169685670469164 in   3.215ms
04: load/parse      in  52.096µs
 1:            1363 in  88.584µs
 2:            8184 in   2.132ms
05: load/parse      in 113.957µs
 1:             520 in  63.331µs
 2: 347338785050515 in 111.000ns
06: load/parse      in  96.959µs
 1:   7326876294741 in   4.432µs
 2:  10756006415204 in  71.285µs
Total elapsed time:      9.085ms
```