#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl Position {
    pub fn increment(&self) -> Position {
        let new_position = self.row * 9 + self.column + 1;

        Position::from(new_position)
    }

    pub fn increment_column(&self) -> Position {
        Position {
            row: self.row,
            column: self.column + 1,
        }
    }

    pub fn increment_row(&self) -> Position {
        Position {
            row: self.row + 1,
            column: self.column,
        }
    }

    pub fn is_valid(&self) -> bool {
        (0..=8).contains(&self.row) && (0..=8).contains(&self.column)
    }

    /// Calculate the 3x3 square as (row, number) where both are in range 0-2
    pub fn calculate_square(&self) -> (usize, usize) {
        let square_row = self.row / 3;
        let square_column = self.column / 3;

        (square_row, square_column)
    }
}

impl From<usize> for Position {
    /// Convert a 1d usize (0-80) into a Position
    fn from(position: usize) -> Self {
        Position {
            row: position / 9,
            column: position % 9,
        }
    }
}

impl From<(usize, usize)> for Position {
    /// Convert a 2d usize of (row, col) into a position
    fn from(coords: (usize, usize)) -> Self {
        Position {
            row: coords.0,
            column: coords.1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn increments_position_from_zero() {
        let position = Position { row: 0, column: 0 };
        assert!(position.is_valid());

        let new_position = position.increment();

        assert_eq!(new_position.column, 1);
        assert_eq!(new_position.row, 0);
        assert!(new_position.is_valid());
    }

    #[test]
    fn goes_beyond_own_limits() {
        let position = Position { row: 8, column: 8 };
        assert!(position.is_valid());

        let new_position = position.increment();

        assert_eq!(new_position.column, 0);
        assert_eq!(new_position.row, 9);
        assert!(!new_position.is_valid());
    }

    #[test]
    fn creates_position_0_from_usize() {
        let input: usize = 0;
        let position: Position = input.into();

        assert_eq!(position.row, 0);
        assert_eq!(position.column, 0);
    }

    #[test]
    fn creates_position_80_from_usize() {
        let input: usize = 80;
        let position: Position = input.into();

        assert_eq!(position.row, 8);
        assert_eq!(position.column, 8);
    }

    #[test]
    fn calculate_square_test() {
        assert_eq!(Position::from((0, 0)).calculate_square(), (0, 0));
        assert_eq!(Position::from((2, 2)).calculate_square(), (0, 0));
        assert_eq!(Position::from((4, 2)).calculate_square(), (1, 0));
        assert_eq!(Position::from((8, 2)).calculate_square(), (2, 0));
        assert_eq!(Position::from((8, 3)).calculate_square(), (2, 1));
        assert_eq!(Position::from((8, 6)).calculate_square(), (2, 2));
    }
}
