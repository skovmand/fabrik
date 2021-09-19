use crate::sudoku::{Position, SudokuBoard, SudokuField};

/// Iterates through a column with a fixed column position and a changing row
pub struct ColumnIter<'a> {
    pub board: &'a SudokuBoard,
    pub position: Position,
}

impl ColumnIter<'_> {
    pub fn new<'a>(board: &'a SudokuBoard, position: &'a Position) -> ColumnIter<'a> {
        ColumnIter {
            board,
            position: Position {
                row: position.row,
                column: 0,
            },
        }
    }
}

impl Iterator for ColumnIter<'_> {
    type Item = SudokuField;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.is_valid() {
            let field = self.board.get_field(&self.position).to_owned();
            self.position = self.position.increment_column();

            return Some(field);
        }

        None
    }
}
