mod sudoku_board;
mod sudoku_field;

use sudoku_board::SudokuBoard;
use sudoku_field::SudokuField;

// Backtracking here is at once very advanced and also really simple!
// For every iteration we are finding the first empty field on the board,
// scanning row, column and square to check if a number is valid,
// then placing the number in the field. After this we will recurse by calling
// the same function once again, but below the recursion we will put back None
// into the field, so if the program backtracks, it will restore the original state.
pub fn solve_board(board: &mut SudokuBoard) -> Result<SudokuBoard, String> {
    match board.first_free_field() {
        // There are no more free fields. The board is solved!
        None => Ok(board.clone()),

        // We are still solving!
        Some((row, column)) => {
            for num in 1..=9 {
                if board.valid_number(row, column, num) {
                    board.put_field(row, column, SudokuField::Value(num));

                    if let Ok(board) = solve_board(board) {
                        return Ok(board);
                    }

                    board.put_field(row, column, SudokuField::Empty);
                }
            }

            Err("The board could not be solved".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::*;
    use indoc::indoc;

    const TEST_SUDOKU: &str = indoc! {"
            -47---96-
            8--716--2
            6-------8
            --21-85--
            ----9----
            --16-23--
            5-------1
            7--945--3
            -69---75-
    "};

    #[test]
    fn solves_board() {
        let mut board = SudokuBoard::try_from(TEST_SUDOKU.to_owned()).unwrap();
        solve_board(&mut board).expect("Could not solve test board");

        println!("{}", board);
    }
}
