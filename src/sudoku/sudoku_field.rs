use std::{
    convert::TryFrom,
    fmt::{Display, Write},
};

use super::SudokuError;

/// A SudokuField is either a number 1-9 or a empty field
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum SudokuField {
    Value(u8),
    Empty,
}

/// Try to parse a an ASCII-encoded byte to a SudokuField
impl TryFrom<&u8> for SudokuField {
    type Error = SudokuError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            b'-' => Ok(SudokuField::Empty),
            val if (49..=57).contains(val) => Ok(SudokuField::Value(val - 48)),
            _ => Err(SudokuError::InvalidCharacterInInput),
        }
    }
}

/// Render a SudokuField as a String
impl Display for SudokuField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SudokuField::Empty => f.write_char(' '),
            SudokuField::Value(val) => f.write_str(&format!("{}", val)),
        }
    }
}

impl SudokuField {
    pub fn is_filled(&self) -> bool {
        matches!(self, SudokuField::Value(_))
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, SudokuField::Empty)
    }
}
