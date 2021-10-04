# fabrik

![Build Status](https://github.com/skovmand/fabrik/workflows/Fabrik%20CI/badge.svg)

A Sudoku solver with backtracking written in Rust ❤️

[![asciicast](https://asciinema.org/a/434516.svg)](https://asciinema.org/a/434516)


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

If the file is called `sudoku-1.txt`, solve it using `fabrik <filename>`. If you pass the `--display` or `-d` flag, the sudoku will be solved with a delay after each step. This is great for learning how backtracking works.

```
# Solve a sudoku in display mode
$ fabrik sudoku-1.txt --display

# Solve a sudoku in display with custom delay (1ms per step)
$ fabrik sudoku-1.txt --display --delay 1

# Solve a sudoku and display the final result
$ fabrik sudoku-1.txt
+-----------+
|632|514|987|
|518|397|624|
|794|268|153|
+---+---+---+
|183|479|265|
|259|136|748|
|476|825|319|
+---+---+---+
|821|743|596|
|947|652|831|
|365|981|472|
+-----------+
```


## Tests

Run the project tests using `cargo test`


## Benchmarks

Run the project benchmarks using `cargo bench`

Results on my MacBook Air M1 2020 (256GB/8GB ram). The "hard sudoku" is a newspaper sudoku with level "hard". The "very hard sudoku" is the hardest one my father-in-law knows :-)

```
solve hard sudoku (1432-3)
                        time:   [146.26 us 146.30 us 146.36 us]

solve very hard sudoku (from ykw1)
                        time:   [365.79 us 365.89 us 366.01 us]
```
