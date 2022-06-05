use super::SudokuRenderer;
use crate::sudoku::SudokuBoard;

pub struct NoopRenderer {}

impl SudokuRenderer for NoopRenderer {
    fn setup(&self, _filename: &str) {}
    fn display_step(&self, _board: &SudokuBoard) {}
    fn display_final_result(&self, _board: &SudokuBoard) {}
    fn teardown(&self) {}
}
