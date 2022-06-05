#![forbid(unsafe_code)]

pub mod renderers;
pub mod sudoku;

use renderers::SudokuRenderer;
use sudoku::{Position, SudokuBoard, SudokuError, SudokuField};

/// Solve a SudokuBoard with callback to a SudokuRenderer at every step
pub fn solve_board(
    board: &mut SudokuBoard,
    renderer: &impl SudokuRenderer,
) -> Result<SudokuBoard, SudokuError> {
    do_solve_board(board, renderer, Position { row: 0, column: 0 })
}

// Solve the Sudoku Board
// Backtracking here is at once very advanced and also really simple!
// For every iteration we are finding the first empty field on the board,
// scanning row, column and square to check if a number is valid,
// then placing the number in the field. After this we will recurse by calling
// the same function once again, but below the recursion we will put back None
// into the field, so if the program backtracks, it will restore the original state.
fn do_solve_board(
    board: &mut SudokuBoard,
    renderer: &impl SudokuRenderer,
    position: Position,
) -> Result<SudokuBoard, SudokuError> {
    for num in 1..=9 {
        let field = SudokuField::Value(num);

        if board.valid_number(&position, &field) {
            board.put_field(&position, field);
            renderer.display_step(board);

            if let Some(position) = board.next_empty_field(&position) {
                // Another free field is present, let's jump to that and continue solving
                if let Ok(board) = do_solve_board(board, renderer, position) {
                    return Ok(board);
                }

                // We met a dead end, set the field to Empty again (after which we will continue looping)
                board.put_field(&position, SudokuField::Empty);
            } else {
                // The sudoku has no more empty fields, we are done!
                return Ok(board.clone());
            };
        }
    }

    Err(SudokuError::Unsolvable)
}

#[cfg(test)]
mod tests {
    use crate::renderers::NonRenderer;

    use super::*;
    use indoc::indoc;

    #[test]
    fn solves_board() {
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

        let mut board = SudokuBoard::try_from(TEST_SUDOKU.to_owned()).unwrap();
        solve_board(&mut board, &NonRenderer {}).expect("Could not solve test board");

        let expected_solution = indoc! {"
            +-----------+
            |147|823|965|
            |895|716|432|
            |623|459|178|
            +---+---+---+
            |972|138|546|
            |386|594|217|
            |451|672|389|
            +---+---+---+
            |534|267|891|
            |718|945|623|
            |269|381|754|
            +-----------+
        "};

        assert_eq!(board.to_string(), expected_solution);
    }

    #[test]
    fn fails_on_invalid_board() {
        const INVALID_SUDOKU: &str = indoc! {"
            -47---96-
            84-716--2
            6-------8
            --21-85--
            ----9----
            --16-23--
            5-------1
            7--945--3
            -69---75-
        "};

        let mut board = SudokuBoard::try_from(INVALID_SUDOKU.to_owned()).unwrap();
        let result = solve_board(&mut board, &NonRenderer {});

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), SudokuError::Unsolvable);
    }

    #[test]
    fn solves_blank_board() {
        const BLANK_SUDOKU: &str = indoc! {"
            ---------
            ---------
            ---------
            ---------
            ---------
            ---------
            ---------
            ---------
            ---------
        "};

        let mut board = SudokuBoard::try_from(BLANK_SUDOKU.to_owned()).unwrap();
        solve_board(&mut board, &NonRenderer {}).expect("Could not solve test board");

        let expected_solution = indoc! {"
            +-----------+
            |123|456|789|
            |456|789|123|
            |789|123|456|
            +---+---+---+
            |214|365|897|
            |365|897|214|
            |897|214|365|
            +---+---+---+
            |531|642|978|
            |642|978|531|
            |978|531|642|
            +-----------+
        "};

        assert_eq!(board.to_string(), expected_solution);
    }

    #[test]
    fn solves_hard_board() {
        const HARD_SUDOKU: &str = indoc! {"
                -3-----8-
                5-------4
                --42-81--
                1-34-92-5
                ---------
                4-68-53-9
                --17-35--
                9-------1
                -6-----7-
        "};

        let mut board = SudokuBoard::try_from(HARD_SUDOKU.to_owned()).unwrap();
        solve_board(&mut board, &NonRenderer {}).expect("Could not solve test board");

        let expected_solution = indoc! {"
            +-----------+
            |632|514|987|
            |518|397|624|
            |794|268|153|
            +---+---+---+
            |183|479|265|
            |259|136|748|
            |476|825|319|
            +---+---+---+
            |821|743|596|
            |947|652|831|
            |365|981|472|
            +-----------+
        "};

        assert_eq!(board.to_string(), expected_solution);
    }
}
