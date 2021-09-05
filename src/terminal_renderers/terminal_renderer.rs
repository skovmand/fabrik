use fabrik::{renderers::SudokuRenderer, sudoku::SudokuBoard};

pub struct TerminalRenderer {}

impl SudokuRenderer for TerminalRenderer {
    fn setup(&self, filename: &str) {
        println!("{}", filename);
    }

    fn display_step(&self, _board: &SudokuBoard) {}

    fn display_final_result(&self, board: &SudokuBoard) {
        print!("{}", board);
    }

    fn teardown(&self) {}
}
