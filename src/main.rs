use std::{convert::TryFrom, fs, thread, time};

use clap::{crate_version, App, AppSettings, Arg};
use fabrik::{solve_board, sudoku_board::SudokuBoard};

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

    if display_mode {
        let delay = matches
            .value_of("delay")
            .map_or(50, |x| x.parse().expect("Could not parse delay value"));

        let sleep_time = time::Duration::from_millis(delay);
        let callback_fn = make_print_sudoku_callback(sleep_time);

        clear_screen();
        hide_cursor();
        cursor_at_position(1, 1);
        println!("Solving {} with {}ms step delay", filename, delay);

        let return_code: i32 = match solve(filename, Some(&callback_fn)) {
            Ok(_board) => 0,
            Err(error) => {
                println!("Error: {}", error);
                1
            }
        };

        show_cursor();
        std::process::exit(return_code);
    }
}

// Dirty little ansi hack to place the cursor where we want it
fn cursor_at_position(row: u8, column: u8) {
    print!("{}[{};{}H", 27 as char, row, column);
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn hide_cursor() {
    print!("{}[?25l", 27 as char);
}

fn show_cursor() {
    print!("{}[?25h", 27 as char);
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

// Helper function to print the sudoku to screen and wait a given interval
// fn print_sudoku(board: &SudokuBoard, sleep_time: std::time::Duration) {
//     // Position the cursor at L1 C1 in the terminal
//     cursor_at_position(3, 1);
//     print!("{}", board);
//     thread::sleep(sleep_time);
// }

fn solve(
    filename: &str,
    callback: Option<&impl Fn(&SudokuBoard)>,
) -> Result<SudokuBoard, Box<dyn std::error::Error>> {
    let sudoku_file = fs::read_to_string(filename)?;
    let mut board = SudokuBoard::try_from(sudoku_file)?;
    solve_board(&mut board, callback)?;

    Ok(board)
}
