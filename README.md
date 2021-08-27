# fabrik

![Build Status](https://github.com/skovmand/fabrik/workflows/Fabrik%20CI/badge.svg)

A Sudoku solver with backtracking written in Rust

## Setup

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone this repository using `git@github.com:skovmand/fabrik.git`
3. Build the binary using `cargo build --release`
4. Copy `./target/release/fabrik` to wherever you like


## Usage

Put a sudoku in a file, with empty fields as `-` and `1-9` as field values. For example

```
-3-----8-
5-------4
--42-81--
1-34-92-5
---------
4-68-53-9
--17-35--
9-------1
-6-----7-
```

If the file is called `sudoku-1.txt`, solve it using `fabrik` like this:

```
$ fabrik sudoku-1.txt
```


## Tests

Run the project tests using `cargo test`


## Benchmarks

Run the project benchmarks using `cargo bench`

Results on my MacBook Air M1 2020 (256GB/8GB ram). The "hard sudoku" is a newspaper sudoku with level "hard". The "very hard sudoku" is the hardest one my father-in-law knows :-)

```
solve hard sudoku (1432-3)
                        time:   [177.54 us 177.59 us 177.64 us]
Found 14 outliers among 100 measurements (14.00%)
  5 (5.00%) high mild
  9 (9.00%) high severe

solve very hard sudoku (from ykw1)
                        time:   [470.87 us 470.97 us 471.10 us]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
```
