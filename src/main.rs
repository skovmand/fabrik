#![forbid(unsafe_code)]

use clap::{crate_version, Arg, Command};
use fabrik::{
    renderers::{DelayedRenderer, Renderer, ResultOnlyRenderer, SudokuRenderer},
    solve_board,
    sudoku::SudokuBoard,
};
use std::{fs, time::Duration};

fn main() {
    let matches = Command::new("fabrik")
        .version(crate_version!())
        .author("https://github.com/skovmand/fabrik")
        .about("Brute force sudoku solver")
        .arg_required_else_help(true)
        .arg(
            Arg::new("display")
                .long("display")
                .short('d')
                .help("Solve the sudoku in display mode"),
        )
        .arg(
            Arg::new("delay")
                .long("delay")
                .takes_value(true)
                .help("Set the delay in ms used in display mode (defaults to 50ms)"),
        )
        .arg(
            Arg::new("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap();

    let renderer: Renderer = if matches.is_present("display") {
        let delay = matches
            .value_of("delay")
            .map_or(50, |x| x.parse().unwrap_or(50));

        let delay = Duration::from_millis(delay);

        Renderer::Delayed(DelayedRenderer { delay })
    } else {
        Renderer::FinalResultOnly(ResultOnlyRenderer {})
    };

    // Set up renderer
    renderer.setup(filename);

    match solve(filename, &renderer) {
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
fn solve<T: SudokuRenderer>(
    filename: &str,
    renderer: &T,
) -> Result<SudokuBoard, Box<dyn std::error::Error>> {
    let sudoku_file = fs::read_to_string(filename)?;
    let mut board = SudokuBoard::try_from(sudoku_file)?;
    solve_board(&mut board, renderer)?;

    Ok(board)
}
