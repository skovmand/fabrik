//! Errors emitted from the fabrik library

use std::collections::HashSet;

use crate::Position;

/// Sudoku board parse-errors
#[derive(Debug, PartialEq)]
pub enum SudokuParseError {
    /// Input does not have length 81
    InvalidLength,
    /// The Sudoku has parse errors
    ParseErrors(HashSet<(Position, FieldParseError)>),
}

impl std::error::Error for SudokuParseError {}

impl std::fmt::Display for SudokuParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SudokuParseError::ParseErrors(_) => write!(
                f,
                "Sudoku has invalid fields or fields that violate the sudoku rules"
            ),
            SudokuParseError::InvalidLength => write!(f, "Input does not have length 81"),
        }
    }
}

/// Sudoku field parse-errors
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FieldParseError {
    /// An invalid character was found in the input
    InvalidCharacter,
    /// The board has rule violations
    SudokuRuleViolation,
}

impl std::error::Error for FieldParseError {}

impl std::fmt::Display for FieldParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FieldParseError::InvalidCharacter => write!(f, "Invalid character"),
            FieldParseError::SudokuRuleViolation => write!(f, "Field violates sudoku rules"),
        }
    }
}

/// Errors from solving a sudoku
#[derive(Debug, PartialEq)]
pub enum SudokuSolveError {
    /// The sudoku does not have a solution
    Unsolvable,
}

impl std::error::Error for SudokuSolveError {}

impl std::fmt::Display for SudokuSolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SudokuSolveError::Unsolvable => write!(f, "The sudoku is unsolvable"),
        }
    }
}
