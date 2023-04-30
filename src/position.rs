/// A position on the sudoku board
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    /// The row in the board, 0-8
    pub(crate) row: usize,
    /// The column in the board, 0-8
    pub(crate) column: usize,
}

impl Position {
    /// Create a position from a 0-based field index of the whole board
    pub(crate) fn from_index(index: usize) -> Position {
        Position {
            row: index / 9,
            column: index % 9,
        }
    }

    /// Create a new position based on the current one, but incremented one field
    pub(crate) fn incremented_copy(&self) -> Option<Position> {
        let coords = self.row * 9 + self.column + 1;

        // Last coord on board is 80
        if coords < 81 {
            Some(Position {
                row: coords / 9,
                column: coords % 9,
            })
        } else {
            None
        }
    }

    /// Get the row
    pub fn row(&self) -> usize {
        self.row
    }

    /// Get the column
    pub fn column(&self) -> usize {
        self.column
    }
}
