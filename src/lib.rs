//! # fabrik
//!
//! An iterator based sudoku solving library using backtracking. Written in Rust ❤️
//!
//! fabrik is implemented as a Rust iterator emitting every single board in the
//! solution process. This allows users of the library to easily build things
//! on top, for example visualizations of the solution process or counting how
//! many guesses a solution took.
//!
//! Built for the fun of learning the principle of backtracking and how it can
//! be implemented in Rust.
//!
//! ## Try an example
//!
//! You can try fabrik by running the provided cli example in a terminal.
//!
//! For instant solutions:
//!
//! ```text
//! cargo run --example cli -- sudokus/oneeighty.txt
//! ```
//!
//! For watching the solution get solved step by step:
//!
//! ```text
//! cargo run --example cli -- sudokus/oneeighty.txt --display
//! ```
//!
//! The same, but with a 5ms delay instead:
//!
//! ```text
//! cargo run --example cli -- sudokus/oneeighty.txt --display --delay 5
//! ```
//!
//! ## Usage
//!
//! ### Get the first solution of a sudoku
//!
//! A well formed sudoku board should only have a single solution. This will
//! return the first solution found.
//!
//! ```rust
//! use fabrik::Board;
//!
//! let nice_sudoku_with_single_solution = "-349---28
//!                                         2-------6
//!                                         ---271---
//!                                         -----2-6-
//!                                         45-----39
//!                                         -6-4-----
//!                                         ---614---
//!                                         3-------1
//!                                         98---364-";
//!
//! let input_board = Board::try_from(nice_sudoku_with_single_solution).expect("Could not parse board");
//! let solution: Board = input_board.first_solution().expect("Board could not be solved");
//!
//! assert_eq!(solution.to_string(), "+-----------+
//! |134|956|728|
//! |275|348|196|
//! |698|271|354|
//! +---+---+---+
//! |819|532|467|
//! |452|167|839|
//! |763|489|512|
//! +---+---+---+
//! |527|614|983|
//! |346|895|271|
//! |981|723|645|
//! +-----------+
//! ");
//! ```
//!
//! ### Solving sudokus using the backtracking iterator
//!
//! Custom logic can be put on top of the backtracking iterator in many ways.
//!
//! This example calculates a vector of all solutions for a board by filtering on `is_solved`:
//!
//! ```rust
//! use fabrik::Board;
//!
//! let i_have_two_solutions = "-349---28
//!                             2-------6
//!                             ---271---
//!                             -----2-6-
//!                             45-----39
//!                             -6-4-----
//!                             ---614---
//!                             3-------1
//!                             98---36--";
//!
//! let input_board = Board::try_from(i_have_two_solutions).expect("Could not parse board");
//!
//! let solutions: Vec<Board> = input_board
//!     .solve_iter()
//!     .filter(|(_, is_solved)| *is_solved)
//!     .map(|(board, _)| board)
//!     .collect::<Vec<Board>>();
//!
//! let solutions_as_strings = solutions
//!     .iter()
//!     .map(|s| s.to_string())
//!     .collect::<Vec<String>>();
//!
//! assert_eq!(solutions_as_strings, vec!["+-----------+
//! |134|956|728|
//! |275|348|196|
//! |698|271|345|
//! +---+---+---+
//! |819|532|467|
//! |452|167|839|
//! |763|489|512|
//! +---+---+---+
//! |527|614|983|
//! |346|895|271|
//! |981|723|654|
//! +-----------+
//! ", "+-----------+
//! |134|956|728|
//! |275|348|196|
//! |698|271|354|
//! +---+---+---+
//! |819|532|467|
//! |452|167|839|
//! |763|489|512|
//! +---+---+---+
//! |527|614|983|
//! |346|895|271|
//! |981|723|645|
//! +-----------+
//! "]);
//! ```
//!
//! As another example, let's calculate the 5th solution for a given board:
//!
//! ```rust
//! use fabrik::Board;
//!
//! let i_have_at_least_five_solutions = "-349---28
//!                                       2-------6
//!                                       ---271---
//!                                       -----2-6-
//!                                       45-----39
//!                                       -6-4-----
//!                                       ---614---
//!                                       3-------1
//!                                       98---364-";
//!
//! let board = Board::try_from(i_have_at_least_five_solutions).expect("Could not parse board");
//!
//! let fifth_solution = board.solve_iter()
//!     .filter(|(_, is_solved)| *is_solved)
//!     .enumerate()
//!     .find(|(index, (_, is_solved))| index == &4);
//! ```
//!
//! Let's also try to find the number of required steps to solve a sudoku:
//!
//! ```rust
//! use fabrik::Board;
//!
//! let nice_sudoku_with_single_solution = "-349---28
//!                                         2-------6
//!                                         ---271---
//!                                         -----2-6-
//!                                         45-----39
//!                                         -6-4-----
//!                                         ---614---
//!                                         3-------1
//!                                         98---364-";
//!
//! let board = Board::try_from(nice_sudoku_with_single_solution).expect("Could not parse board");
//!
//! let step_count = board.solve_iter()
//!     .enumerate()
//!     .find(|(_, (_, is_solved))| *is_solved)
//!     .map(|(index, _)| index);
//!
//! assert_eq!(step_count, Some(228));
//! ```
//!
//! ### Counting solutions
//!
//! fabrik has a `count_solutions` helper to easily count solutions for a board.
//! Counting solutions has dragons since an empty board will have a very large
//! solution count, and will take a very long time to compute!
//! For this reason `count_solutions` has the option to set maximum solutions to
//! count, and maximum iterations allowed before bailing.
//!
//! This example counts the number of solutions for a sudoku board:
//!
//! ```rust
//! use fabrik::Board;
//!
//! let i_have_multiple_solutions = "6-------4
//!                                  -42-3-51-
//!                                  -85---32-
//!                                  ---3-5---
//!                                  53-----68
//!                                  ---6-2---
//!                                  -26-5-89-
//!                                  -97---45-
//!                                  1-------2";
//!
//! let input_board = Board::try_from(i_have_multiple_solutions).expect("Could not parse board");
//!
//! // Counting without limits on solutions and iterations
//! assert_eq!(input_board.count_solutions(None, None), 21);
//!
//! // Counting the same board with max solutions of 10
//! assert_eq!(input_board.count_solutions(Some(10), None), 10);
//!
//! // Counting the same board with max iterations of 10_000
//! assert_eq!(input_board.count_solutions(None, Some(10_000)), 13);
//! ```
//!
//! ## Notes on invariance
//!
//! By design fabrik ensures that the data structures used are always valid. it is not possible to:
//!
//! - Create an invalid sudoku board
//! - Modify a valid sudoku board into an invalid sudoku board
//! - Represent out of bounds positions on the sudoku board
//! - Represent single board fields with invalid values
//! - Break the sudoku rule of only one allowed unique number per row, column, square
//!
//!## Benchmarks
//!
//!Run the project benchmarks using `cargo bench`
//!
//!Results on my MacBook Pro M1 2021 based on the following three sudokus in the `sudokus/` folder:
//!
//!```text
//!Benchmarking solve one-eighty sudoku (easy):
//!                        time:   [13.751 µs 13.768 µs 13.790 µs]
//!
//!Benchmarking solve starry sudoku (medium):
//!                        time:   [183.78 µs 183.88 µs 183.99 µs]
//!
//!Benchmarking solve turbine (hard):
//!                        time:   [393.26 µs 393.42 µs 393.61 µs]
//!```

#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::expect_used,
    clippy::match_wildcard_for_single_variants,
    clippy::needless_borrow,
    clippy::todo,
    clippy::unwrap_used,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    unused
)]
#![forbid(unsafe_code)]
#![deny(private_in_public)]

mod backtracking_iter;
mod board;
mod error;
mod field;
mod position;
mod position_iter;

// Public API
pub use {
    board::Board,
    error::{SudokuParseError, SudokuSolveError},
    field::Field,
    position::Position,
    position_iter::PositionIter,
};
