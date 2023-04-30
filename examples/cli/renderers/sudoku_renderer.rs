use fabrik::Board;

pub trait SudokuRenderer {
    fn setup(&self, filename: &str);
    fn display_step(&self, board: &Board);
    fn display_final_result(&self, board: &Board);
    fn teardown(&self);
}
