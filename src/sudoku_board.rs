use crate::sudoku_field::SudokuField;
use std::{convert::TryFrom, fmt::Display};

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

#[cfg(test)]
mod tests {
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
    fn reads_a_board_from_file() {
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
    fn fails_to_read_a_board_from_file_with_wrong_length() {
        let file_plus_one = format!("{}1", TEST_SUDOKU);
        let board = SudokuBoard::try_from(file_plus_one);

        assert!(board.is_err());
        assert_eq!(board.err().unwrap(), "Input does not have length 81");
    }

    #[test]
    fn fails_to_read_a_board_from_file_with_invalid_char() {
        let file_plus_one = format!("{}A", TEST_SUDOKU);
        let board = SudokuBoard::try_from(file_plus_one);

        assert!(board.is_err());
        assert_eq!(board.err().unwrap(), "Invalid character 'A' in input");
    }
}
