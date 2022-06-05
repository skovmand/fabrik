use crate::sudoku::{Position, SudokuBoard, SudokuField};

/// Iterator that emits a tuple of (Position, SudokuField).
/// The current position of the iterator will be the next value emitted.
pub struct BoardIter<'a> {
    board: &'a SudokuBoard,
    position: Position,
}

impl<'a> BoardIter<'a> {
    pub fn new(board: &SudokuBoard) -> BoardIter {
        BoardIter {
            board,
            position: Position { row: 0, column: 0 },
        }
    }

    pub fn set_position(&mut self, position: &Position) -> &'a mut BoardIter {
        self.position = *position;
        self
    }
}

impl Iterator for BoardIter<'_> {
    type Item = (Position, SudokuField);

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.is_valid() {
            let value = *self.board.get_field(&self.position);
            let position = self.position;
            self.position = self.position.increment();

            return Some((position, value));
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_SUDOKU: &str = "
            -47---96-\n
            8--716--2\n
            6-------8\n
            --21-85--\n
            ----9----\n
            --16-23--\n
            5-------1\n
            7--945--3\n
            -69---75-";

    #[test]
    fn iterator_with_position_emits_fields_in_expected_order() {
        let board = SudokuBoard::try_from(TEST_SUDOKU.to_string()).unwrap();
        let mut iterator = BoardIter {
            board: &board,
            position: Position { row: 0, column: 0 },
        };

        for &value in &[
            (Position { row: 0, column: 0 }, SudokuField::Empty),
            (Position { row: 0, column: 1 }, SudokuField::Value(4)),
            (Position { row: 0, column: 2 }, SudokuField::Value(7)),
            (Position { row: 0, column: 3 }, SudokuField::Empty),
        ] {
            assert_eq!(iterator.next(), Some(value));
        }

        // Advance 75 fields
        for _val in 0..75 {
            iterator.next();
        }

        for &value in &[
            (Position { row: 8, column: 7 }, SudokuField::Value(5)),
            (Position { row: 8, column: 8 }, SudokuField::Empty),
        ] {
            assert_eq!(iterator.next(), Some(value));
        }

        assert!(iterator.next().is_none());
    }
}
