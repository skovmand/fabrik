#[derive(Debug, PartialEq)]
pub enum SudokuError {
    InvalidLength,
    InvalidCharacterInInput,
    Unsolvable,
}

impl std::error::Error for SudokuError {}

impl std::fmt::Display for SudokuError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SudokuError::InvalidCharacterInInput => write!(f, "Invalid character in input"),
            SudokuError::InvalidLength => write!(f, "Input does not have length 81"),
            SudokuError::Unsolvable => write!(f, "The sudoku is unsolvable"),
        }
    }
}
