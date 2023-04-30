use std::fmt::{Display, Write};

use crate::error::FieldParseError;

/// Represents a field value on a sudoku board.
///
/// The `Field` can either have a value of 1-9, or be empty.
#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub struct Field(FieldInner);

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
enum FieldInner {
    /// A filled in sudoku field with a value 1-9
    Value(u8),
    /// An empty field
    Empty,
}

impl Field {
    /// Create an empty `Field`
    pub fn empty() -> Self {
        Field(FieldInner::Empty)
    }

    /// Create a `Field` with value. Does not validate the value
    pub(crate) fn from_u8(value: u8) -> Self {
        Field(FieldInner::Value(value))
    }

    /// Create a `Field` from a digit 1-9
    pub fn new(digit: u8) -> Result<Field, FieldParseError> {
        if (1..=9).contains(&digit) {
            Ok(Field(FieldInner::Value(digit)))
        } else {
            Err(FieldParseError::InvalidCharacter)
        }
    }

    /// Get the value of a `Field` if set, otherwise `None` if empty
    pub fn value(&self) -> Option<u8> {
        match self.0 {
            FieldInner::Value(value) => Some(value),
            FieldInner::Empty => None,
        }
    }

    /// Is the `Field` filled?
    pub fn is_filled(&self) -> bool {
        matches!(self, Field(FieldInner::Value(_)))
    }

    /// Is the `Field` empty?
    pub fn is_empty(&self) -> bool {
        matches!(self, Field(FieldInner::Empty))
    }
}

/// Render a `Field` as a `String`
impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field(FieldInner::Empty) => f.write_char(' '),
            Field(FieldInner::Value(val)) => f.write_str(&format!("{val}")),
        }
    }
}
