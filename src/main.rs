mod ansi;

use std::{convert::TryFrom, fs, thread, time};

use clap::{crate_version, App, AppSettings, Arg};
use fabrik::{solve_board, sudoku_board::SudokuBoard};

use crate::ansi::*;

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
        .arg(
            Arg::new("delay")
                .long("delay")
                .takes_value(true)
                .about("Set the delay in ms used in display mode (defaults to 50)"),
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::new("INPUT")
                .about("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Get the values of cmd-line args
    let filename = matches.value_of("INPUT").unwrap();
    let display_mode: bool = matches.is_present("display");
    let delay: u64 = matches
        .value_of("delay")
        .map_or(50, |x| x.parse().expect("Could not parse delay value"));

    let return_code = match {
        if display_mode {
            solve_in_display_mode(filename, delay)
        } else {
            solve_in_standard_mode(filename)
        }
    } {
        Ok(_board) => 0,
        Err(error) => {
            println!("Error: {}", error);
            1
        }
    };

    std::process::exit(return_code);
}

fn solve_in_display_mode(
    filename: &str,
    delay: u64,
) -> Result<SudokuBoard, Box<dyn std::error::Error>> {
    let sleep_time = time::Duration::from_millis(delay);
    let callback_fn = make_print_sudoku_callback(sleep_time);

    clear_screen();
    hide_cursor();
    cursor_at_position(1, 1);
    println!("Solving {} with {}ms step delay", filename, delay);

    let result = solve(filename, Some(&callback_fn));
    show_cursor();
    result
}

fn solve_in_standard_mode(filename: &str) -> Result<SudokuBoard, Box<dyn std::error::Error>> {
    solve(filename, None)
}

// Helper function to print the sudoku to screen and wait a given interval
fn make_print_sudoku_callback(sleep_time: std::time::Duration) -> impl Fn(&SudokuBoard) {
    move |board| {
        // Position the cursor at L1 C1 in the terminal
        cursor_at_position(3, 1);
        print!("{}", board);
        thread::sleep(sleep_time);
    }
}

// Solve a sudoku given an optional callback
fn solve(
    filename: &str,
    callback: Option<&dyn Fn(&SudokuBoard)>,
) -> Result<SudokuBoard, Box<dyn std::error::Error>> {
    let sudoku_file = fs::read_to_string(filename)?;
    let mut board = SudokuBoard::try_from(sudoku_file)?;
    solve_board(&mut board, callback)?;

    Ok(board)
}
