use std::convert::TryFrom;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use fabrik::{solve_board, sudoku_board::SudokuBoard};

pub fn criterion_benchmark(c: &mut Criterion) {
    const HARD_SUDOKU: &str = "
                -3-----8-\n
                5-------4\n
                --42-81--\n
                1-34-92-5\n
                ---------\n
                4-68-53-9\n
                --17-35--\n
                9-------1\n
                -6-----7-";

    const VERY_HARD_SUDOKU: &str = "
                --15----3\n
                ----18-2-\n
                -----7--1\n
                --7-6---4\n
                -9-8-1-6-\n
                2---4-1--\n
                5--6-----\n
                -8-32----\n
                3----45--\n
                ";

    c.bench_function("solve hard sudoku (1432-3)", |b| {
        b.iter_batched(
            || {
                let input = HARD_SUDOKU.to_owned();
                SudokuBoard::try_from(input).unwrap()
            },
            |mut board| solve_board(&mut board).unwrap(),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("solve very hard sudoku (from ykw1)", |b| {
        b.iter_batched(
            || {
                let input = VERY_HARD_SUDOKU.to_owned();
                SudokuBoard::try_from(input).unwrap()
            },
            |mut board| solve_board(&mut board).unwrap(),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
