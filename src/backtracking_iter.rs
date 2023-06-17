use crate::Board;

use super::{field::Field, position::Position};

/// Iterator emitting `Board`s representing each steps towards a solved
/// sudoku using a backtracking algorithm.
///
/// The iterator will emit all possible boards for the input until it finally
/// emits None, which means all possibilities have been tried for the given
/// input board.
#[derive(Debug)]
pub struct BacktrackingIter {
    board: Board,
    current_position: Position,
    stack: Vec<Instruction>,
}

enum WhatHappened {
    PutNewFieldOnBoard,
    RanOutOfStack,
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    TryValue(Position, u8),
    BackTo(Position),
}

impl BacktrackingIter {
    /// Create a backtracking iterator for a Board
    pub fn new(board: Board) -> Self {
        BacktrackingIter {
            current_position: Position { row: 0, column: 0 },
            board,
            stack: Vec::new(),
        }
    }

    // Prepare instructions in the stack for execution
    fn prepare_stack(&mut self, next_empty_field: Position) {
        // Insert a BackTo(position) with the current position
        self.stack.push(Instruction::BackTo(self.current_position));

        // Try the value 1 first. This will be incremented up until 9 during execution.
        // We could have pushed 9 separate instructions instead, but this performs better.
        self.stack.push(Instruction::TryValue(next_empty_field, 1));
    }

    // Manipulate the board from the stack instructions
    fn execute_stack(&mut self) -> WhatHappened {
        loop {
            match self.stack.pop() {
                Some(instruction) => match instruction {
                    Instruction::TryValue(pos, v) => {
                        self.current_position = pos;

                        for value in v..=9 {
                            let field = Field::from_u8(value);

                            if self.board.valid_number_at_position(pos, &field) {
                                // Insert TryValue(v + 1) on the top of the stack, to be able to resume work on this field
                                // if we backtrack to this position again. But only if (v + 1 <= 9).
                                if value < 9 {
                                    self.stack.push(Instruction::TryValue(pos, value + 1));
                                }

                                self.board.put_field(pos, field);
                                return WhatHappened::PutNewFieldOnBoard;
                            }
                        }

                        // Nothing is returned, which means what we will loop once more
                    }
                    Instruction::BackTo(pos) => {
                        // Clear out the current position before moving back
                        self.board.put_field(self.current_position, Field::empty());

                        // Move back
                        self.current_position = pos;
                    }
                },
                None => {
                    return WhatHappened::RanOutOfStack;
                }
            }
        }
    }
}

impl Iterator for BacktrackingIter {
    type Item = (Board, bool);

    fn next(&mut self) -> Option<Self::Item> {
        // If there's a next empty field, prepare the stack for that field, otherwise
        // just keep executing the stack.
        if let Some(next_empty_field) = self.board.next_empty_field(self.current_position) {
            self.prepare_stack(next_empty_field);
        }

        match self.execute_stack() {
            WhatHappened::PutNewFieldOnBoard => {
                // After the new field is put on the board,
                // Check to see if more fields are available
                // If not, then we consider the board solved
                let are_more_fields_available =
                    self.board.next_empty_field(self.current_position).is_none();

                Some((self.board, are_more_fields_available))
            }
            WhatHappened::RanOutOfStack => None,
        }
    }
}
