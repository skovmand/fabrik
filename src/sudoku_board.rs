use crate::sudoku_field::SudokuField;
use std::{convert::TryFrom, fmt::Display};

#[derive(Clone)]
pub struct SudokuBoard(Vec<SudokuField>);

/// Read a String into a board
impl TryFrom<String> for SudokuBoard {
    type Error = String;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        let trimmed_input: Vec<SudokuField> = input
            .split_whitespace()
            .collect::<String>()
            .as_bytes()
            .iter()
            .map(SudokuField::try_from)
            .collect::<Result<Vec<SudokuField>, String>>()?;

        if trimmed_input.len() == 81 {
            Ok(SudokuBoard(trimmed_input))
        } else {
            Err("Input does not have length 81".into())
        }
    }
}

/// Get a String representation of a board
impl Display for SudokuBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+-----------+")?;

        for row in 0..=8 {
            write!(f, "|")?;

            for column in 0..=8 {
                write!(f, "{}", self.0[row * 9 + column])?;

                if (column + 1) % 3 == 0 {
                    write!(f, "|")?;
                }
            }

            writeln!(f)?;

            if (row + 1) % 3 == 0 && row != 8 {
                writeln!(f, "+---+---+---+")?;
            }
        }

        writeln!(f, "+-----------+")?;

        Ok(())
    }
}

impl SudokuBoard {
    /// Get the value of a field at row and column
    pub fn get_field(&self, row: usize, column: usize) -> &SudokuField {
        &self.0[row * 9 + column]
    }

    /// Update a field on the board
    pub fn put_field(&mut self, row: usize, column: usize, sudoku_field: SudokuField) {
        self.0[row * 9 + column] = sudoku_field;
    }

    /// Get the first free field of the board as (row, column)
    pub fn first_free_field(&self) -> Option<(usize, usize)> {
        for row in 0..9 {
            for column in 0..9 {
                if self.get_field(row, column).is_empty() {
                    return Some((row, column));
                }
            }
        }

        None
    }

    /// Is a number valid at a given position?
    pub fn valid_number(&self, row: usize, column: usize, number: u8) -> bool {
        let number = SudokuField::Value(number);

        !self.number_used_in_row(row, &number)
            && !self.number_used_in_column(column, &number)
            && !self.number_used_in_square(row, column, &number)
    }

    /// Is a number unique in a horizontal row?
    fn number_used_in_row(&self, row: usize, number: &SudokuField) -> bool {
        for column in 0..9 {
            if number == self.get_field(row, column) {
                return true;
            }
        }

        false
    }

    /// Is a number unique in a horizontal row?
    fn number_used_in_column(&self, column: usize, number: &SudokuField) -> bool {
        for row in 0..9 {
            if number == self.get_field(row, column) {
                return true;
            }
        }

        false
    }

    /// Is a number used in a 3x3 square?
    fn number_used_in_square(&self, row: usize, column: usize, number: &SudokuField) -> bool {
        let (square_row, square_col) = calculate_square(row, column);

        for current_row in (square_row * 3)..(square_row * 3 + 3) {
            for current_col in (square_col * 3)..(square_col * 3 + 3) {
                if self.get_field(current_row, current_col) == number {
                    return true;
                }
            }
        }

        false
    }
}

/// Calculate the 3x3 square as (row, number) where both are in range 0-2
fn calculate_square(row: usize, column: usize) -> (usize, usize) {
    let square_row = row / 3;
    let square_column = column / 3;

    (square_row, square_column)
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;
    use indoc::indoc;

    const TEST_SUDOKU: &str = indoc! {"
            -47---96-
            8--716--2
            6-------8
            --21-85--
            ----9----
            --16-23--
            5-------1
            7--945--3
            -69---75-
    "};

    #[test]
    fn reads_a_board_from_string() {
        let board = SudokuBoard::try_from(TEST_SUDOKU.to_owned()).unwrap();

        let expected_board = indoc! {"
        +-----------+
        | 47|   |96 |
        |8  |716|  2|
        |6  |   |  8|
        +---+---+---+
        |  2|1 8|5  |
        |   | 9 |   |
        |  1|6 2|3  |
        +---+---+---+
        |5  |   |  1|
        |7  |945|  3|
        | 69|   |75 |
        +-----------+
        "};

        assert_eq!(board.to_string(), expected_board);
    }

    #[test]
    fn fails_to_read_a_board_from_string_with_wrong_length() {
        let file_plus_one = format!("{}1", TEST_SUDOKU);
        let board = SudokuBoard::try_from(file_plus_one);

        assert!(board.is_err());
        assert_eq!(board.err().unwrap(), "Input does not have length 81");
    }

    #[test]
    fn fails_to_read_a_board_from_string_with_invalid_char() {
        let file_plus_one = format!("{}A", TEST_SUDOKU);
        let board = SudokuBoard::try_from(file_plus_one);

        assert!(board.is_err());
        assert_eq!(board.err().unwrap(), "Invalid character 'A' in input");
    }

    #[test]
    fn first_empty_field() {
        let mut board = SudokuBoard::try_from(TEST_SUDOKU.to_owned()).unwrap();
        assert_eq!(board.first_free_field(), Some((0, 0)));

        board.put_field(0, 0, (&b'8').try_into().unwrap());
        assert_eq!(board.first_free_field(), Some((0, 3)));

        // Fill entire column with garbage
        for column in 0..9 {
            board.put_field(0, column, (&b'9').try_into().unwrap());
        }

        assert_eq!(board.first_free_field(), Some((1, 1)));

        // Fill entire board with garbage numbers
        for row in 0..9 {
            for column in 0..9 {
                board.put_field(row, column, (&b'9').try_into().unwrap());
            }
        }

        assert_eq!(board.first_free_field(), None);
    }

    #[test]
    fn number_used_in_row() {
        let board = SudokuBoard::try_from(TEST_SUDOKU.to_owned()).unwrap();

        assert!(board.number_used_in_row(0, &SudokuField::Value(4)));
        assert!(!board.number_used_in_row(0, &SudokuField::Value(5)));
        assert!(board.number_used_in_row(6, &SudokuField::Value(5)));
        assert!(!board.number_used_in_row(6, &SudokuField::Value(3)));
    }

    #[test]
    fn number_used_in_column() {
        let board = SudokuBoard::try_from(TEST_SUDOKU.to_owned()).unwrap();

        assert!(board.number_used_in_column(2, &SudokuField::Value(7)));
        assert!(!board.number_used_in_column(2, &SudokuField::Value(3)));
        assert!(board.number_used_in_column(8, &SudokuField::Value(1)));
        assert!(!board.number_used_in_column(8, &SudokuField::Value(9)));
    }

    #[test]
    fn number_used_in_square() {
        let board = SudokuBoard::try_from(TEST_SUDOKU.to_owned()).unwrap();

        assert!(board.number_used_in_square(0, 0, &SudokuField::Value(7)));
        assert!(!board.number_used_in_square(0, 0, &SudokuField::Value(1)));
        assert!(board.number_used_in_square(1, 2, &SudokuField::Value(8)));
        assert!(!board.number_used_in_square(1, 2, &SudokuField::Value(5)));
    }

    #[test]
    fn calculate_square_test() {
        assert_eq!(calculate_square(0, 0), (0, 0));
        assert_eq!(calculate_square(2, 2), (0, 0));
        assert_eq!(calculate_square(4, 2), (1, 0));
        assert_eq!(calculate_square(8, 2), (2, 0));
        assert_eq!(calculate_square(8, 3), (2, 1));
        assert_eq!(calculate_square(8, 6), (2, 2));
    }

    #[test]
    fn valid_number() {
        let board = SudokuBoard::try_from(TEST_SUDOKU.to_owned()).unwrap();
        assert!(!board.valid_number(2, 2, 9));
        assert!(!board.valid_number(8, 8, 1));
        assert!(!board.valid_number(3, 3, 1));
        assert!(board.valid_number(3, 3, 3));
        assert!(board.valid_number(0, 0, 2));
        assert!(board.valid_number(7, 7, 2));
    }
}
