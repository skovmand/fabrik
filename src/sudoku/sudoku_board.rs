use std::{convert::TryFrom, fmt::Display};

use super::{position::Position, SudokuError, SudokuField};

#[derive(Clone)]
pub struct SudokuBoard([[SudokuField; 9]; 9]);

/// Read a String into a board
impl TryFrom<String> for SudokuBoard {
    type Error = SudokuError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        let trimmed_input = input.split_whitespace().collect::<String>();

        if trimmed_input.len() != 81 {
            return Err(SudokuError::InvalidLength);
        }

        let mut board_as_array = [[SudokuField::Empty; 9]; 9];

        for (i, field) in trimmed_input.as_bytes().iter().enumerate() {
            let row = i / 9;
            let column = i - row * 9;

            board_as_array[row][column] = SudokuField::try_from(field)?;
        }

        Ok(SudokuBoard(board_as_array))
    }
}

/// Get a String representation of a board
impl Display for SudokuBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+-----------+")?;

        for row in 0..=8 {
            write!(f, "|")?;

            for column in 0..=8 {
                write!(f, "{}", self.0[row][column])?;

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
    pub fn get_field(&self, position: &Position) -> &SudokuField {
        &self.0[position.0][position.1]
    }

    /// Update a field on the board
    pub fn put_field(&mut self, position: &Position, sudoku_field: SudokuField) {
        self.0[position.0][position.1] = sudoku_field;
    }

    /// Get the first free field of the board as (row, column)
    pub fn first_free_field(&self) -> Option<Position> {
        for row in 0..9 {
            for column in 0..9 {
                let position = Position(row, column);

                if self.get_field(&position).is_empty() {
                    return Some(position);
                }
            }
        }

        None
    }

    /// Is a number valid at a given position?
    pub fn valid_number(&self, position: &Position, number: &SudokuField) -> bool {
        !self.number_used_in_row(position, number)
            && !self.number_used_in_column(position, number)
            && !self.number_used_in_square(position, number)
    }

    /// Is a number unique in a horizontal row?
    fn number_used_in_row(&self, position: &Position, number: &SudokuField) -> bool {
        let row = self.0[position.0];

        for field in row.iter() {
            if field == number {
                return true;
            }
        }

        false
    }

    /// Is a number unique in a horizontal row?
    fn number_used_in_column(&self, position: &Position, number: &SudokuField) -> bool {
        for row in 0..9 {
            let position = Position(row, position.1);

            if number == self.get_field(&position) {
                return true;
            }
        }

        false
    }

    /// Is a number used in a 3x3 square?
    fn number_used_in_square(&self, position: &Position, number: &SudokuField) -> bool {
        let (square_row, square_col) = calculate_square(position.0, position.1);

        for row in (square_row * 3)..(square_row * 3 + 3) {
            for column in (square_col * 3)..(square_col * 3 + 3) {
                let position = Position(row, column);
                if self.get_field(&position) == number {
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

        assert_eq!(board.err().unwrap(), SudokuError::InvalidLength);
    }

    #[test]
    fn fails_to_read_a_board_from_string_with_invalid_char() {
        let invalid_board = format!("Q{}", &TEST_SUDOKU[1..]);
        let board = SudokuBoard::try_from(invalid_board);

        assert!(board.is_err());

        assert_eq!(board.err().unwrap(), SudokuError::InvalidCharacterInInput);
    }

    #[test]
    fn first_empty_field() {
        let mut board = SudokuBoard::try_from(TEST_SUDOKU.to_owned()).unwrap();
        assert_eq!(board.first_free_field(), Some(Position(0, 0)));

        board.put_field(&Position(0, 0), (&b'8').try_into().unwrap());
        assert_eq!(board.first_free_field(), Some(Position(0, 3)));

        // Fill entire column with garbage
        for column in 0..9 {
            board.put_field(&Position(0, column), (&b'9').try_into().unwrap());
        }

        assert_eq!(board.first_free_field(), Some(Position(1, 1)));

        // Fill entire board with garbage numbers
        for row in 0..9 {
            for column in 0..9 {
                board.put_field(&Position(row, column), (&b'9').try_into().unwrap());
            }
        }

        assert_eq!(board.first_free_field(), None);
    }

    #[test]
    fn number_used_in_row() {
        let board = SudokuBoard::try_from(TEST_SUDOKU.to_owned()).unwrap();

        assert!(board.number_used_in_row(&Position(0, 0), &SudokuField::Value(4)));
        assert!(!board.number_used_in_row(&Position(0, 0), &SudokuField::Value(5)));
        assert!(board.number_used_in_row(&Position(6, 0), &SudokuField::Value(5)));
        assert!(!board.number_used_in_row(&Position(6, 0), &SudokuField::Value(3)));
    }

    #[test]
    fn number_used_in_column() {
        let board = SudokuBoard::try_from(TEST_SUDOKU.to_owned()).unwrap();

        assert!(board.number_used_in_column(&Position(0, 2), &SudokuField::Value(7)));
        assert!(!board.number_used_in_column(&Position(0, 2), &SudokuField::Value(3)));
        assert!(board.number_used_in_column(&Position(0, 8), &SudokuField::Value(1)));
        assert!(!board.number_used_in_column(&Position(0, 8), &SudokuField::Value(9)));
    }

    #[test]
    fn number_used_in_square() {
        let board = SudokuBoard::try_from(TEST_SUDOKU.to_owned()).unwrap();

        assert!(board.number_used_in_square(&Position(0, 0), &SudokuField::Value(7)));
        assert!(!board.number_used_in_square(&Position(0, 0), &SudokuField::Value(1)));
        assert!(board.number_used_in_square(&Position(1, 2), &SudokuField::Value(8)));
        assert!(!board.number_used_in_square(&Position(1, 2), &SudokuField::Value(5)));
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
        assert!(!board.valid_number(&Position(2, 2), &SudokuField::Value(9)));
        assert!(!board.valid_number(&Position(8, 8), &SudokuField::Value(1)));
        assert!(!board.valid_number(&Position(3, 3), &SudokuField::Value(1)));
        assert!(board.valid_number(&Position(3, 3), &SudokuField::Value(3)));
        assert!(board.valid_number(&Position(0, 0), &SudokuField::Value(2)));
        assert!(board.valid_number(&Position(7, 7), &SudokuField::Value(2)));
    }
}
