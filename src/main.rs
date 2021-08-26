use std::{convert::TryFrom, fs};

use clap::{crate_version, App, AppSettings, Arg};
use fabrik::{
    solve_board,
    sudoku_board::{SudokuBoard, SudokuError},
};

fn main() -> Result<(), SudokuError> {
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

    let sudoku_file = fs::read_to_string(filename).map_err(|_| SudokuError::FileError)?;
    let mut board = SudokuBoard::try_from(sudoku_file)?;
    solve_board(&mut board)?;

    println!("{}", board);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_calculate() {
        let value = 4;
        assert_eq!(value, 4);
    }
}
