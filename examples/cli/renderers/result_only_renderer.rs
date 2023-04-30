use fabrik::Board;

use crate::renderers::SudokuRenderer;

pub struct ResultOnlyRenderer {}

impl SudokuRenderer for ResultOnlyRenderer {
    fn setup(&self, filename: &str) {
        println!("{filename}");
    }

    fn display_step(&self, _board: &Board) {}

    fn display_final_result(&self, board: &Board) {
        print!("{board}");
    }

    fn teardown(&self) {}
}
