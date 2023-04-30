use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use fabrik::Board;

pub fn criterion_benchmark(c: &mut Criterion) {
    pub const ONEEIGHTY: &str = include_str!("../sudokus/oneeighty.txt");
    pub const STARRY: &str = include_str!("../sudokus/starry.txt");
    pub const TURBINE: &str = include_str!("../sudokus/turbine.txt");

    c.bench_function("solve one-eighty (easy)", |b| {
        b.iter_batched(
            || Board::try_from(ONEEIGHTY).unwrap(),
            |board| board.first_solution().unwrap(),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("solve starry (medium)", |b| {
        b.iter_batched(
            || Board::try_from(STARRY).unwrap(),
            |board| board.first_solution().unwrap(),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("solve turbine (hard)", |b| {
        b.iter_batched(
            || Board::try_from(TURBINE).unwrap(),
            |board| board.first_solution().unwrap(),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
