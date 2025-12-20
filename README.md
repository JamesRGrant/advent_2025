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
01: load/parse      in 464.430µs
 1:            1177 in  18.072µs
 2:            6768 in  28.546µs
02: load/parse      in  44.523µs
 1:     17077011375 in   2.609ms
 2:     36037497037 in  76.000ns
03: load/parse      in  46.476µs
 1:           17155 in  14.360µs
 2: 169685670469164 in  47.290µs
04: load/parse      in 161.149µs
 1:            1363 in  54.000ns
 2:            8184 in 585.637µs
05: load/parse      in 105.816µs
 1:             520 in  62.411µs
 2: 347338785050515 in 108.000ns
06: load/parse      in 100.725µs
 1:   7326876294741 in   5.156µs
 2:  10756006415204 in  79.820µs
07: load/parse      in  43.915µs
 1:            1598 in  56.593µs
 2:   4509723641302 in 264.370µs
08: load/parse      in  33.084ms
 1:           84968 in 685.287µs
 2:      8663467782 in   1.706ms
09: load/parse      in   7.691ms
 1:      4759420470 in 227.000ns
 2:      1603439684 in  37.155ms
10: load/parse      in 297.464µs
 1:             415 in   2.840ms
 2:             156 in 162.000ns
11: load/parse      in   2.090ms
 1:             523 in   8.140µs
 2: 517315308154944 in  92.313µs
12: load/parse      in   8.227µs
 1:               0 in  54.000ns
 2:               0 in  33.000ns
Total elapsed time:     93.006ms
```

Three problmems are unfinished.