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
01: load/parse      in 464.189µs
 1:            1177 in  16.382µs
 2:            6768 in  25.352µs
02: load/parse      in  41.558µs
 1:     17077011375 in   9.656ms
 2:     36037497037 in 157.000ns
03: load/parse      in  37.303µs
 1:           17155 in  38.057µs
 2: 169685670469164 in   3.252ms
04: load/parse      in  53.950µs
 1:            1363 in  86.969µs
 2:            8184 in   2.115ms
05: load/parse      in 108.252µs
 1:             520 in  55.437µs
 2: 347338785050515 in 147.000ns
06: load/parse      in  93.874µs
 1:   7326876294741 in   4.157µs
 2:  10756006415204 in  69.462µs
Total elapsed time:     16.393ms
```