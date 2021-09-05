use super::SudokuRenderer;
use crate::sudoku_board::SudokuBoard;

pub struct NonRenderer {}

impl SudokuRenderer for NonRenderer {
    fn setup(&self, _filename: &str) {}
    fn display_step(&self, _board: &SudokuBoard) {}
    fn display_final_result(&self, _board: &SudokuBoard) {}
    fn teardown(&self) {}
}
