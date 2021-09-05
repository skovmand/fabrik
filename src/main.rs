mod ansi;
mod terminal_renderers;

use std::{convert::TryFrom, fs};

use clap::{crate_version, App, AppSettings, Arg};
use fabrik::{renderers::SudokuRenderer, solve_board, sudoku_board::SudokuBoard};

use crate::terminal_renderers::{DelayedRenderer, TerminalRenderer};

fn main() {
    let matches = App::new("fabrik")
        .version(crate_version!())
        .author("https://github.com/skovmand/fabrik")
        .about("Brute force sudoku solver using backtracking")
        .arg(
            Arg::new("display")
                .long("display")
                .short('d')
                .about("Solve the sudoku in display mode"),
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::new("INPUT")
                .about("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap();
    let renderer: &dyn SudokuRenderer = if matches.is_present("display") {
        &DelayedRenderer {}
    } else {
        &TerminalRenderer {}
    };

    // Set up renderer
    renderer.setup(filename);

    match solve(filename, renderer) {
        Ok(board) => {
            renderer.display_final_result(&board);
            renderer.teardown();
            std::process::exit(0);
        }
        Err(error) => {
            println!("Error: {}", error);
            renderer.teardown();
            std::process::exit(1);
        }
    };
}

// Solve the sudoku given an optional callback
fn solve(
    filename: &str,
    renderer: &dyn SudokuRenderer,
) -> Result<SudokuBoard, Box<dyn std::error::Error>> {
    let sudoku_file = fs::read_to_string(filename)?;
    let mut board = SudokuBoard::try_from(sudoku_file)?;
    solve_board(&mut board, renderer)?;

    Ok(board)
}
