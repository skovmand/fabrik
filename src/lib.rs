mod sudoku_board;
mod sudoku_field;

use sudoku_board::SudokuBoard;

// Backtracking here is at once very advanced and also really simple!
// For every iteration we are finding the first empty field on the board,
// scanning row, column and square to check if a number is valid,
// then placing the number in the field. After this we will recurse by calling
// the same function once again, but below the recursion we will put back None
// into the field, so if the program backtracks, it will restore the original state.
pub fn solve_board(_board: &mut SudokuBoard) -> Result<SudokuBoard, String> {
    //    match board.first_free_field() {
    //        // There are no more free fields. The board is solved!
    //        None => Ok(board),
    //
    //        // We are still solving!
    //        Some((row, column)) => {
    //            for num in 1..=9 {}
    //            return Ok(board);
    //            if let Ok(board) = solve_board(&board) {
    //                return Ok(board);
    //            } else {
    //                // Put back the value
    //            }
    //        }
    //    }
    //
    //    Err("The board could not be solved")

    todo!()
}
