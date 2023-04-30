use std::{collections::HashSet, fmt::Display};

use crate::{error::FieldParseError, position_iter::PositionIter, SudokuSolveError};

use super::{
    backtracking_iter::BacktrackingIter, error::SudokuParseError, field::Field, position::Position,
};

/// The Sudoku Board
///
/// The board always contains valid fields and cannot violate the sudoku rules,
/// for example it is never possible to have the same digit twice in a square.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Board([[Field; 9]; 9]);

impl Board {
    ////////////////
    // Accessors //
    //////////////

    /// Get the `Field` at a given `Position`
    pub fn get_field(&self, position: Position) -> &Field {
        &self.0[position.row][position.column]
    }

    /// Put a `Field` on the `Board`
    pub(crate) fn put_field(&mut self, position: Position, sudoku_field: Field) {
        self.0[position.row][position.column] = sudoku_field;
    }

    /// Given a `Position`, get the next free `Field`
    pub(crate) fn next_empty_field(&self, position: Position) -> Option<Position> {
        PositionIter::new(position).find(|position| self.get_field(*position).is_empty())
    }

    ////////////////
    // Solutions //
    //////////////

    /// Get the first solution for a `Board`
    pub fn first_solution(self) -> Result<Board, SudokuSolveError> {
        if let Some(solution) = self
            .solve_iter()
            .find(|(_, is_solved)| *is_solved)
            .map(|(board, _)| board)
        {
            Ok(solution)
        } else {
            Err(SudokuSolveError::Unsolvable)
        }
    }

    /// Count solutions for a `Board`
    ///
    /// An almost empty sudoku will have many solutions, and calculating them all will
    /// be an expensive operation. For that reason it is possible to specify a maximum
    /// for both total iterations and total solutions counted.
    ///
    /// For instance, the consumer might not be interested in counting more than 100
    /// solutions.
    pub fn count_solutions(
        self,
        max_solutions: Option<usize>,
        max_iterations: Option<usize>,
    ) -> usize {
        self.solve_iter()
            .enumerate()
            .take_while(|(index, _)| {
                // Maximum iterations allowed
                if let Some(max) = max_iterations {
                    index < &max
                } else {
                    true
                }
            })
            .filter(|(_, (_, is_solved))| *is_solved)
            .enumerate()
            .take_while(|(index, _)| {
                // Maximum solutions allowed
                if let Some(max) = max_solutions {
                    index < &max
                } else {
                    true
                }
            })
            .count()
    }

    /// Iterator emitting `(board: Board, is_solved: Bool)` on the way towards
    /// a solution using the backtracking technique
    pub fn solve_iter(self) -> BacktrackingIter {
        BacktrackingIter::new(self)
    }

    /////////////////
    // Validation //
    ///////////////

    /// Do any digits in the `Board` violate the sudoku rules? For instance it is not
    /// valid to have the digit 5 twice in a row on the board.
    fn rule_violations(self) -> HashSet<Position> {
        PositionIter::from_first_field()
            .map(|position| (position, self.get_field(position)))
            .filter(|(_, field)| field.is_filled())
            .filter(|(pos, field)| {
                let mut temp_board = self;
                temp_board.put_field(*pos, Field::empty());
                !temp_board.valid_number_at_position(*pos, field)
            })
            .map(|(pos, _)| pos)
            .collect::<HashSet<Position>>()
    }

    /// Is a number valid at a given position?
    /// Note: This assumes the field is not in the board yet
    pub(crate) fn valid_number_at_position(&self, position: Position, number: &Field) -> bool {
        !self.number_used_in_row(position, number)
            && !self.number_used_in_column(position, number)
            && !self.number_used_in_square(position, number)
    }

    /// Is a number currently used in a row?
    fn number_used_in_row(&self, position: Position, number: &Field) -> bool {
        let row_slice = &self.0[position.row];
        row_slice.iter().any(|field| field == number)
    }

    /// Is a number currently used in a column?
    fn number_used_in_column(&self, position: Position, number: &Field) -> bool {
        (0..9)
            .map(|row| Position {
                row,
                column: position.column,
            })
            .any(|position| self.get_field(position) == number)
    }

    /// Is a number used in a 3x3 square?
    fn number_used_in_square(&self, position: Position, number: &Field) -> bool {
        let square_row = position.row / 3;
        let square_column = position.column / 3;

        (0..3)
            .map(|row_increase| {
                &self.0[square_row * 3 + row_increase][(square_column * 3)..(square_column * 3 + 3)]
            })
            .any(|slice| slice.contains(number))
    }
}

/// Create a `Board`  from a `String`
impl TryFrom<String> for Board {
    type Error = SudokuParseError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        Board::try_from(input.as_str())
    }
}

/// Create a `Board` from a `str`
impl TryFrom<&str> for Board {
    type Error = SudokuParseError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let input_vector = input
            .split_whitespace()
            .collect::<String>()
            .bytes()
            .collect::<Vec<u8>>();

        Board::try_from(input_vector)
    }
}

/// Create a `Board` from a vector of bytes
impl TryFrom<Vec<u8>> for Board {
    type Error = SudokuParseError;

    fn try_from(input: Vec<u8>) -> Result<Self, Self::Error> {
        let prepared_vec = input
            .iter()
            .map(|c| match c {
                b'-' | b' ' => None,
                val if (49..=57).contains(val) => Some(val - 48),
                _ => Some(255), // Use an invalid field value which will fail in the next step
            })
            .collect::<Vec<Option<u8>>>();

        Board::try_from(prepared_vec)
    }
}

/// Create a `Board` from a vector of `Option<u8>` where the u8 is a Some with digit 1-9
/// and empty fields are represented as None
impl TryFrom<Vec<Option<u8>>> for Board {
    type Error = SudokuParseError;

    fn try_from(input: Vec<Option<u8>>) -> Result<Self, Self::Error> {
        if input.len() != 81 {
            return Err(SudokuParseError::InvalidLength);
        }

        // 1. Build up a board, treating invalid fields as empty fields,
        //    while inserting them as validation errors in the HashSet.
        let mut lenient_board = Board([[Field::empty(); 9]; 9]);
        let mut positions_with_parse_errors = HashSet::new();

        for (i, field) in input.iter().enumerate() {
            let position = Position::from_index(i);

            let parsed_field = match field {
                Some(val) => match Field::new(*val) {
                    Ok(field) => field,
                    Err(_) => {
                        positions_with_parse_errors
                            .insert((position, FieldParseError::InvalidCharacter));
                        Field::empty()
                    }
                },
                None => Field::empty(),
            };

            lenient_board.put_field(position, parsed_field);
        }

        // 2. Check the leniently parsed board for rule violations
        let rule_violations = lenient_board
            .rule_violations()
            .iter()
            .map(|pos| (*pos, FieldParseError::SudokuRuleViolation))
            .collect::<HashSet<(Position, FieldParseError)>>();

        let all_errors = positions_with_parse_errors
            .union(&rule_violations)
            .cloned()
            .collect::<HashSet<(Position, FieldParseError)>>();

        // If no errors, the board is valid
        if all_errors.is_empty() {
            Ok(lenient_board)
        } else {
            Err(SudokuParseError::ParseErrors(all_errors))
        }
    }
}

/// Get a `String` representation of a `Board`
impl Display for Board {
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

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod accessor_tests {
    use super::*;

    #[test]
    fn gets_a_field() {
        let board = Board::try_from(
            "1--------
             -2-------
             --3------
             ---4-----
             ----5----
             -----6---
             ------7--
             -------8-
             --------9
        ",
        )
        .unwrap();

        assert_eq!(
            board.get_field(Position { row: 0, column: 0 }),
            &Field::from_u8(1)
        );

        assert_eq!(
            board.get_field(Position { row: 8, column: 8 }),
            &Field::from_u8(9)
        );

        assert_eq!(
            board.get_field(Position { row: 8, column: 7 }),
            &Field::empty()
        );
    }

    #[test]
    fn puts_a_field() {
        let mut board = Board([[Field::empty(); 9]; 9]);

        assert_eq!(
            board.get_field(Position { row: 1, column: 3 }),
            &Field::empty()
        );

        board.put_field(Position { row: 1, column: 3 }, Field::from_u8(2));

        assert_eq!(
            board.get_field(Position { row: 1, column: 3 }),
            &Field::from_u8(2)
        );
    }

    #[test]
    fn returns_next_empty_field() {
        // The board is "sudokus/starry.txt" solved except for one field
        let mut board = Board::try_from(
            "613529784
             742836519
             985174326
             269385147
             53194-268
             874612935
             426751893
             397268451
             158493672",
        )
        .unwrap();

        assert_eq!(
            board.next_empty_field(Position { row: 0, column: 4 }),
            Some(Position { row: 4, column: 5 })
        );

        board.put_field(Position { row: 4, column: 5 }, Field::from_u8(7));

        assert!(board
            .next_empty_field(Position { row: 0, column: 4 })
            .is_none());
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod solution_tests {
    use super::*;

    #[test]
    fn finds_first_solution_on_board() {
        // The board is "sudokus/starry.txt"
        let board = Board::try_from(
            "6-------4
             -42-3-51-
             -85---32-
             ---3-5---
             53--4--68
             ---6-2---
             -26-5-89-
             -97---45-
             1-------2",
        )
        .unwrap();

        let solution_count = board.count_solutions(None, None);
        assert_eq!(solution_count, 1);

        let solved_board = board.first_solution().expect("Could not solve test board");

        let solution = Board::try_from(
            "613529784
             742836519
             985174326
             269385147
             531947268
             874612935
             426751893
             397268451
             158493672",
        )
        .unwrap();

        assert_eq!(solved_board, solution);
    }

    #[test]
    fn getting_first_solution_fails_on_unsolveable_board() {
        // The board is "sudokus/starry.txt", but with an added 7 in the center
        let board = Board::try_from(
            "6-------4
             -42-3-51-
             -85---32-
             ---3-5---
             53--4--68
             ---672---
             -26-5-89-
             -97---45-
             1-------2",
        )
        .unwrap();

        let solution_count = board.count_solutions(None, None);
        assert_eq!(solution_count, 0);

        let result = board.first_solution();

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), SudokuSolveError::Unsolvable);
    }

    #[test]
    fn count_solutions_returns_a_single_solution() {
        // The board is "sudokus/turbine.txt"
        let board = Board::try_from(
            "-1-79----
             -3-5---91
             --91--5--
             ------182
             1---2---4
             248------
             --6--92--
             32---4-7-
             ----31-6-",
        )
        .unwrap();

        assert_eq!(board.count_solutions(None, None), 1);
    }

    // The board is "sudokus/starry.txt", but the center 4 is removed
    const STARRY_MULTIPLE_SOLUTIONS: &str = "6-------4
                                             -42-3-51-
                                             -85---32-
                                             ---3-5---
                                             53-----68
                                             ---6-2---
                                             -26-5-89-
                                             -97---45-
                                             1-------2";

    #[test]
    fn count_solutions_returns_multiple_solutions() {
        let board = Board::try_from(STARRY_MULTIPLE_SOLUTIONS).unwrap();
        assert_eq!(board.count_solutions(None, None), 21);
    }

    #[test]
    fn count_solutions_respects_max_solutions() {
        let board = Board::try_from(STARRY_MULTIPLE_SOLUTIONS).unwrap();
        assert_eq!(board.count_solutions(Some(10), None), 10);
    }

    #[test]
    fn count_solutions_respects_max_iterations() {
        let board = Board::try_from(STARRY_MULTIPLE_SOLUTIONS).unwrap();
        assert_eq!(board.count_solutions(None, Some(10_000)), 13);
    }

    #[test]
    fn count_solutions_respects_both_max_iterations_and_max_solutions() {
        let board = Board::try_from(STARRY_MULTIPLE_SOLUTIONS).unwrap();

        // 10_000 iterations will yield 13 solutions, however we set max 10
        assert_eq!(board.count_solutions(Some(10), Some(10_000)), 10);

        // 10_000 iterations will yield 13 solutions, however we set max 20
        assert_eq!(board.count_solutions(Some(15), Some(10_000)), 13);
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod validation_tests {
    use super::*;

    // The "sudokus/starry.txt" board
    const TEST_SUDOKU: &str = "6-------4
                               -42-3-51-
                               -85---32-
                               ---3-5---
                               53--4--68
                               ---6-2---
                               -26-5-89-
                               -97---45-
                               1-------2";

    #[test]
    fn number_used_in_row() {
        let board = Board::try_from(TEST_SUDOKU).unwrap();

        assert!(board.number_used_in_row(Position { row: 0, column: 0 }, &Field::from_u8(4)));
        assert!(!board.number_used_in_row(Position { row: 0, column: 0 }, &Field::from_u8(5)));
        assert!(board.number_used_in_row(Position { row: 6, column: 0 }, &Field::from_u8(5)));
        assert!(!board.number_used_in_row(Position { row: 6, column: 0 }, &Field::from_u8(3)));
    }

    #[test]
    fn number_used_in_column() {
        let board = Board::try_from(TEST_SUDOKU).unwrap();

        assert!(board.number_used_in_column(Position { row: 0, column: 2 }, &Field::from_u8(7)));
        assert!(!board.number_used_in_column(Position { row: 0, column: 2 }, &Field::from_u8(3)));
        assert!(board.number_used_in_column(Position { row: 0, column: 8 }, &Field::from_u8(4)));
        assert!(!board.number_used_in_column(Position { row: 0, column: 8 }, &Field::from_u8(9)));
    }

    #[test]
    fn number_used_in_square() {
        let board = Board::try_from(TEST_SUDOKU).unwrap();

        assert!(board.number_used_in_square(Position { row: 0, column: 0 }, &Field::from_u8(6)));
        assert!(!board.number_used_in_square(Position { row: 0, column: 0 }, &Field::from_u8(1)));
        assert!(board.number_used_in_square(Position { row: 1, column: 8 }, &Field::from_u8(1)));
        assert!(!board.number_used_in_square(Position { row: 1, column: 8 }, &Field::from_u8(6)));
    }

    #[test]
    fn valid_number_at_position() {
        let board = Board::try_from(TEST_SUDOKU).unwrap();

        assert!(!board.valid_number_at_position(Position { row: 0, column: 2 }, &Field::from_u8(8)));
        assert!(!board.valid_number_at_position(Position { row: 0, column: 2 }, &Field::from_u8(7)));
        assert!(!board.valid_number_at_position(Position { row: 0, column: 2 }, &Field::from_u8(4)));
        assert!(board.valid_number_at_position(Position { row: 0, column: 2 }, &Field::from_u8(1)));
        assert!(board.valid_number_at_position(Position { row: 0, column: 2 }, &Field::from_u8(3)));
        assert!(board.valid_number_at_position(Position { row: 0, column: 2 }, &Field::from_u8(9)));
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod to_and_from_string_test {
    use super::*;

    // The "sudokus/oneeighty.txt" board
    const TEST_SUDOKU: &str = "-349---28
                               2-------6
                               ---271---
                               -----2-6-
                               45-----39
                               -6-4-----
                               ---614---
                               3-------1
                               98---364-";

    #[test]
    fn fails_to_read_a_board_from_string_with_wrong_length() {
        let file_plus_one = format!("{TEST_SUDOKU}1");
        let board = Board::try_from(file_plus_one);

        assert!(board.is_err());
        assert_eq!(board.err().unwrap(), SudokuParseError::InvalidLength);
    }

    #[test]
    fn returns_rule_violations_and_invalid_chars() {
        // The "sudokus/oneeighty.txt" board modified
        const TEST_SUDOKU: &str = "-349--328
                                   2-------6
                                   3--271---
                                   -----2-6-
                                   45-----39
                                   -6-4-----
                                   ---614--f
                                   -3------1
                                   98---364-";

        let board = Board::try_from(TEST_SUDOKU);

        assert!(board.is_err());

        let expected_violations = {
            let mut violations = HashSet::new();
            violations.insert((
                Position { row: 0, column: 6 },
                FieldParseError::SudokuRuleViolation,
            ));
            violations.insert((
                Position { row: 0, column: 1 },
                FieldParseError::SudokuRuleViolation,
            ));
            violations.insert((
                Position { row: 2, column: 0 },
                FieldParseError::SudokuRuleViolation,
            ));
            violations.insert((
                Position { row: 7, column: 1 },
                FieldParseError::SudokuRuleViolation,
            ));
            violations.insert((
                Position { row: 6, column: 8 },
                FieldParseError::InvalidCharacter,
            ));

            violations
        };

        assert_eq!(
            board.err().unwrap(),
            SudokuParseError::ParseErrors(expected_violations)
        );
    }

    #[test]
    fn from_vec_full_cycle() {
        let board = Board::try_from(vec![
            None,
            Some(3),
            Some(4),
            Some(9),
            None,
            None,
            None,
            Some(2),
            Some(8),
            Some(2),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(6),
            None,
            None,
            None,
            Some(2),
            Some(7),
            Some(1),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(2),
            None,
            Some(6),
            None,
            Some(4),
            Some(5),
            None,
            None,
            None,
            None,
            None,
            Some(3),
            Some(9),
            None,
            Some(6),
            None,
            Some(4),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(6),
            Some(1),
            Some(4),
            None,
            None,
            None,
            Some(3),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(1),
            Some(9),
            Some(8),
            None,
            None,
            None,
            Some(3),
            Some(6),
            Some(4),
            None,
        ])
        .unwrap();

        let expected_board = "+-----------+
| 34|9  | 28|
|2  |   |  6|
|   |271|   |
+---+---+---+
|   |  2| 6 |
|45 |   | 39|
| 6 |4  |   |
+---+---+---+
|   |614|   |
|3  |   |  1|
|98 |  3|64 |
+-----------+
";

        assert_eq!(board.to_string(), expected_board);
    }

    #[test]
    fn to_string_full_cycle() {
        let board = Board::try_from(TEST_SUDOKU).unwrap();

        let expected_board = "+-----------+
| 34|9  | 28|
|2  |   |  6|
|   |271|   |
+---+---+---+
|   |  2| 6 |
|45 |   | 39|
| 6 |4  |   |
+---+---+---+
|   |614|   |
|3  |   |  1|
|98 |  3|64 |
+-----------+
";

        assert_eq!(board.to_string(), expected_board);
    }
}
