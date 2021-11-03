#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl From<usize> for Position {
    /// Convert a 1d usize into a position
    fn from(coords: usize) -> Self {
        Position {
            row: coords / 9,
            column: coords % 9,
        }
    }
}

impl From<(usize, usize)> for Position {
    /// Convert a 2d usize of (row, col) into a position
    fn from((row, column): (usize, usize)) -> Self {
        Position { row, column }
    }
}

impl Position {
    pub fn increment(&self) -> Position {
        let new_position = self.row * 9 + self.column + 1;

        Position::from(new_position)
    }

    pub fn is_valid(&self) -> bool {
        (0..=8).contains(&self.row) && (0..=8).contains(&self.column)
    }
}
