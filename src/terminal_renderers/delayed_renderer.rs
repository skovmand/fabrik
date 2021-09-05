use fabrik::{renderers::SudokuRenderer, sudoku::SudokuBoard};
use std::{thread, time};

use crate::terminal_renderers::ansi::*;

const SLEEP_TIME: time::Duration = time::Duration::from_millis(50);

pub struct DelayedRenderer {}

impl SudokuRenderer for DelayedRenderer {
    fn setup(&self, filename: &str) {
        clear_screen();
        hide_cursor();
        cursor_at_position(1, 1);
        println!("Solving {} with 50ms step delay", filename);
    }

    // Display the result after a single step
    fn display_step(&self, board: &SudokuBoard) {
        cursor_at_position(3, 1);
        print!("{}", board);
        thread::sleep(SLEEP_TIME);
    }

    // Since the delayed renderer will end up with a solved sudoku
    fn display_final_result(&self, _board: &SudokuBoard) {}

    fn teardown(&self) {
        show_cursor();
    }
}
