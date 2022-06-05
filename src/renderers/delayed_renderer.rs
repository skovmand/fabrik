use crate::renderers::ansi_util::*;
use crate::renderers::SudokuRenderer;
use crate::sudoku::SudokuBoard;

use std::{thread, time::Duration};

pub struct DelayedRenderer {
    pub delay: Duration,
}

impl SudokuRenderer for DelayedRenderer {
    fn setup(&self, filename: &str) {
        clear_screen();
        hide_cursor();
        cursor_at_position(1, 1);
        println!("Solving {} with {:?} step delay", filename, self.delay);
    }

    // Display the result after a single step
    fn display_step(&self, board: &SudokuBoard) {
        cursor_at_position(3, 1);
        print!("{}", board);
        thread::sleep(self.delay);
    }

    // Since the delayed renderer will end up with a solved sudoku using display_step,
    // we will not display the final result
    fn display_final_result(&self, _board: &SudokuBoard) {}

    fn teardown(&self) {
        show_cursor();
    }
}
