use crate::sudoku::{Position, SudokuBoard, SudokuField};

/// Iterates through a column with a fixed column position and a changing row
pub struct RowIter<'a> {
    pub board: &'a SudokuBoard,
    pub position: Position,
}

impl RowIter<'_> {
    pub fn new<'a>(board: &'a SudokuBoard, position: &'a Position) -> RowIter<'a> {
        RowIter {
            board,
            position: Position {
                row: 0,
                column: position.column,
            },
        }
    }
}

impl Iterator for RowIter<'_> {
    type Item = SudokuField;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.is_valid() {
            let field = self.board.get_field(&self.position).to_owned();
            self.position = self.position.increment_row();

            return Some(field);
        }

        None
    }
}
