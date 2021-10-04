use std::{convert::TryFrom, fmt::Display};

use super::{iter::BoardIter, position::Position, SudokuError, SudokuField};

#[derive(Clone)]
pub struct SudokuBoard([[SudokuField; 9]; 9]);

impl SudokuBoard {
    /// Get the value of a field at row and column
    pub fn get_field(&self, position: &Position) -> &SudokuField {
        &self.0[position.row][position.column]
    }

    /// Update a field on the board
    pub fn put_field(&mut self, position: &Position, sudoku_field: SudokuField) {
        self.0[position.row][position.column] = sudoku_field;
    }

    /// Get the next free field of the board
    pub fn next_empty_field(&self, position: &Position) -> Option<Position> {
        BoardIter::new(self, *position)
            .find(|(_position, field)| field.is_empty())
            .map(|(position, _field)| position)
    }

    /// Is a number valid at a given position?
    pub fn valid_number(&self, position: &Position, number: &SudokuField) -> bool {
        !self.number_used_in_row(position, number)
            && !self.number_used_in_column(position, number)
            && !self.number_used_in_square(position, number)
    }

    /// Is a number currently used in a row?
    fn number_used_in_row(&self, position: &Position, number: &SudokuField) -> bool {
        let row_slice = &self.0[position.row];
        row_slice.iter().any(|field| field == number)
    }

    /// Is a number currently used in a column?
    fn number_used_in_column(&self, position: &Position, number: &SudokuField) -> bool {
        (0..9)
            .map(|row| Position::from((row, position.column)))
            .any(|position| self.get_field(&position) == number)
    }

    /// Is a number used in a 3x3 square?
    fn number_used_in_square(&self, position: &Position, number: &SudokuField) -> bool {
        let square_row = position.row / 3;
        let square_column = position.column / 3;

        (0..3)
            .map(|row_increase| {
                &self.0[square_row * 3 + row_increase][(square_column * 3)..(square_column * 3 + 3)]
            })
            .any(|slice| slice.contains(number))
    }
}

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
    fn next_empty_field() {
        let mut board = SudokuBoard::try_from(TEST_SUDOKU.to_owned()).unwrap();
        assert_eq!(board.next_empty_field(&(0, 0).into()), Some((0, 0).into()));

        board.put_field(&(0, 0).into(), (&b'8').try_into().unwrap());
        assert_eq!(board.next_empty_field(&(0, 0).into()), Some((0, 3).into()));

        // Fill entire column with garbage
        for column in 0..9 {
            board.put_field(&(0, column).into(), (&b'9').try_into().unwrap());
        }

        assert_eq!(board.next_empty_field(&(0, 0).into()), Some((1, 1).into()));

        // Fill entire board with garbage numbers
        for row in 0..9 {
            for column in 0..9 {
                board.put_field(&(row, column).into(), (&b'9').try_into().unwrap());
            }
        }

        assert_eq!(board.next_empty_field(&(0, 0).into()), None);
    }

    #[test]
    fn number_used_in_row() {
        let board = SudokuBoard::try_from(TEST_SUDOKU.to_owned()).unwrap();

        assert!(board.number_used_in_row(&(0, 0).into(), &SudokuField::Value(4)));
        assert!(!board.number_used_in_row(&(0, 0).into(), &SudokuField::Value(5)));
        assert!(board.number_used_in_row(&(6, 0).into(), &SudokuField::Value(5)));
        assert!(!board.number_used_in_row(&(6, 0).into(), &SudokuField::Value(3)));
    }

    #[test]
    fn number_used_in_column() {
        let board = SudokuBoard::try_from(TEST_SUDOKU.to_owned()).unwrap();

        assert!(board.number_used_in_column(&(0, 2).into(), &SudokuField::Value(7)));
        assert!(!board.number_used_in_column(&(0, 2).into(), &SudokuField::Value(3)));
        assert!(board.number_used_in_column(&(0, 8).into(), &SudokuField::Value(1)));
        assert!(!board.number_used_in_column(&(0, 8).into(), &SudokuField::Value(9)));
    }

    #[test]
    fn number_used_in_square() {
        let board = SudokuBoard::try_from(TEST_SUDOKU.to_owned()).unwrap();

        assert!(board.number_used_in_square(&(0, 0).into(), &SudokuField::Value(7)));
        assert!(!board.number_used_in_square(&(0, 0).into(), &SudokuField::Value(1)));
        assert!(board.number_used_in_square(&(1, 2).into(), &SudokuField::Value(8)));
        assert!(!board.number_used_in_square(&(1, 2).into(), &SudokuField::Value(5)));
    }

    #[test]
    fn valid_number() {
        let board = SudokuBoard::try_from(TEST_SUDOKU.to_owned()).unwrap();
        assert!(!board.valid_number(&(2, 2).into(), &SudokuField::Value(9)));
        assert!(!board.valid_number(&(8, 8).into(), &SudokuField::Value(1)));
        assert!(!board.valid_number(&(3, 3).into(), &SudokuField::Value(1)));
        assert!(board.valid_number(&(3, 3).into(), &SudokuField::Value(3)));
        assert!(board.valid_number(&(0, 0).into(), &SudokuField::Value(2)));
        assert!(board.valid_number(&(7, 7).into(), &SudokuField::Value(2)));
    }
}
