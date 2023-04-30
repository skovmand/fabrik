use crate::position::Position;

/// Iterator for fields on a board. Useful for iterating all board fields one-by-one.
#[derive(Debug)]
pub struct PositionIter {
    position: Option<Position>,
}

impl PositionIter {
    /// Create a new PositionIter starting at a position
    pub fn new(position: Position) -> Self {
        PositionIter {
            position: Some(position),
        }
    }

    /// Create a new PositionIter starting at the first board field
    pub fn from_first_field() -> Self {
        PositionIter::new(Position { row: 0, column: 0 })
    }
}

impl Iterator for PositionIter {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(position) = self.position {
            self.position = position.incremented_copy();
            Some(position)
        } else {
            None
        }
    }
}

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn emits_fields_in_expected_order() {
        let mut iterator = PositionIter::from_first_field();

        for &value in &[
            (Position { row: 0, column: 0 }),
            (Position { row: 0, column: 1 }),
            (Position { row: 0, column: 2 }),
        ] {
            assert_eq!(iterator.next(), Some(value));
        }

        for _val in 0..75 {
            iterator.next();
        }

        for &value in &[
            (Position { row: 8, column: 6 }),
            (Position { row: 8, column: 7 }),
            (Position { row: 8, column: 8 }),
        ] {
            assert_eq!(iterator.next(), Some(value));
        }

        assert!(iterator.next().is_none());
    }
}
