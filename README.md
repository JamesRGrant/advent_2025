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
01: load/parse      in   4.271ms
 1:            1177 in  37.064µs
 2:            6768 in  55.155µs
02: load/parse      in 565.422µs
 1:     17077011375 in   4.959ms
 2:     36037497037 in 190.000ns
03: load/parse      in 869.612µs
 1:           17155 in  57.099µs
 2: 169685670469164 in 102.286µs
04: load/parse      in 983.408µs
 1:            1363 in 280.000ns
 2:            8184 in   1.042ms
05: load/parse      in 712.040µs
 1:             520 in 131.467µs
 2: 347338785050515 in 290.000ns
06: load/parse      in 915.736µs
 1:   7326876294741 in   9.185µs
 2:  10756006415204 in 139.060µs
07: load/parse      in 697.089µs
 1:            1598 in 111.242µs
 2:   4509723641302 in 372.674µs
08: load/parse      in  60.613ms
 1:           84968 in   1.091ms
 2:      8663467782 in   2.929ms
09: load/parse      in  14.547ms
 1:      4759420470 in 281.000ns
 2:      1603439684 in  74.341ms
10: load/parse      in   2.231ms
 1:             415 in   5.184ms
 2:               0 in 441.000ns
Total elapsed time:    181.693ms
```