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
01: load/parse      in 426.915µs
 1:            1177 in  15.860µs
 2:            6768 in  42.922µs
02: load/parse      in  41.770µs
 1:     17077011375 in   2.358ms
 2:     36037497037 in  60.000ns
03: load/parse      in  43.072µs
 1:           17155 in  13.365µs
 2: 169685670469164 in  46.599µs
04: load/parse      in  48.362µs
 1:            1363 in  89.962µs
 2:            8184 in   2.393ms
05: load/parse      in 100.381µs
 1:             520 in  66.727µs
 2: 347338785050515 in 100.000ns
06: load/parse      in  95.853µs
 1:   7326876294741 in   4.398µs
 2:  10756006415204 in  69.733µs
Total elapsed time:      6.159ms
```