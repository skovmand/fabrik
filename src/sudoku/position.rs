#[derive(Debug, PartialEq, Eq)]
/// Model for a position on a SudokuBoard given as Position(row, column)
/// where row and column are in range 0..=8
pub struct Position(pub usize, pub usize);
