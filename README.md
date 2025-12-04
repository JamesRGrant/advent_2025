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
01: load/parse      in 603.837µs
 1:            1177 in  19.808µs
 2:            6768 in  30.769µs
02: load/parse      in  45.817µs
 1:     17077011375 in  10.290ms
 2:     36037497037 in 151.000ns
03: load/parse      in  44.995µs
 1:           17155 in  43.593µs
 2: 169685670469164 in   3.312ms
04: load/parse      in  30.317µs
 1:               0 in  60.000ns
 2:               0 in  30.000ns
Total elapsed time:     14.722ms
```