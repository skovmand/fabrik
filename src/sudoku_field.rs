use std::{
    convert::TryFrom,
    fmt::{Display, Write},
};

/// A SudokuField is either a number 1-9 or a empty field
pub enum SudokuField {
    Value(u8),
    Empty,
}

/// Try to parse a &u8 to a SudokuField
impl TryFrom<&u8> for SudokuField {
    type Error = String;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            b'-' => Ok(SudokuField::Empty),
            val if (49..=57).contains(val) => Ok(SudokuField::Value(val - 48)),
            other => Err(format!("Invalid character '{}' in input", *other as char)),
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
