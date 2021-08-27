use std::{convert::TryFrom, fs};

use clap::{crate_version, App, AppSettings, Arg};
use fabrik::{solve_board, sudoku_board::SudokuBoard};

fn main() {
    let matches = App::new("fabrik")
        .version(crate_version!())
        .author("https://github.com/skovmand/fabrik")
        .about("Brute force sudoku solver using backtracking")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::new("INPUT")
                .about("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Since the INPUT arg is required, we use unwrap
    let filename = matches.value_of("INPUT").unwrap();

    match solve(filename) {
        Ok(board) => {
            print!("{}", board);
            std::process::exit(0);
        }
        // Handle any error here, however it must implement std::error::Error,
        // which means it implements the Display Trait, and therefore it can be printed as a string
        Err(error) => {
            println!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

fn solve(filename: &str) -> Result<SudokuBoard, Box<dyn std::error::Error>> {
    let sudoku_file = fs::read_to_string(filename)?;
    let mut board = SudokuBoard::try_from(sudoku_file)?;
    solve_board(&mut board)?;

    Ok(board)
}
