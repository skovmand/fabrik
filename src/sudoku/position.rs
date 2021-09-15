#[derive(Debug, PartialEq, Eq)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl From<usize> for Position {
    /// Convert a 1d usize (0-80) into a Position
    fn from(position: usize) -> Self {
        Position {
            row: position / 9,
            column: position / 9,
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

impl From<Position> for usize {
    fn from(val: Position) -> Self {
        val.row * 9 + val.column
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
    fn converts_min_position_to_usize() {
        let position = Position { row: 0, column: 0 };
        let usize: usize = position.into();

        assert_eq!(usize, 0);
    }

    #[test]
    fn converts_max_position_to_usize() {
        let position = Position { row: 8, column: 8 };
        let usize: usize = position.into();

        assert_eq!(usize, 80);
    }
}
