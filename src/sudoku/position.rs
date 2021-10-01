#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl From<(usize, usize)> for Position {
    /// Convert a 2d usize of (row, col) into a position
    fn from(coords: (usize, usize)) -> Self {
        Position {
            row: coords.0,
            column: coords.1,
        }
    }
}
