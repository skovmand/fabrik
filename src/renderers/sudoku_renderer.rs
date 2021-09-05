use crate::sudoku_board::SudokuBoard;

pub trait SudokuRenderer {
    fn setup(&self, filename: &str);
    fn display_step(&self, board: &SudokuBoard);
    fn display_final_result(&self, board: &SudokuBoard);
    fn teardown(&self);
}
