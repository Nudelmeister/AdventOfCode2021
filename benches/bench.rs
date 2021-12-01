use aoc2021::p01_sonarsweep;
use criterion::{criterion_group, criterion_main, Criterion};

criterion_main!(bench);
criterion_group!(
    name = bench;
    config = Criterion::default().sample_size(1_000);
    targets = benchmark_01_sonarsweep,
);

fn benchmark_01_sonarsweep(c: &mut Criterion) {
    let input = p01_sonarsweep::read_input("inputs/01-sonarsweep.txt").collect::<Vec<_>>();
    let mut group = c.benchmark_group("Puzzle 01 - Sonar Sweep");
    group.bench_function("solve_loop", |b| {
        b.iter_batched(
            || input.clone().into_iter(),
            p01_sonarsweep::solve_loop,
            criterion::BatchSize::SmallInput,
        );
    });
    group.bench_function("solve_iter", |b| {
        b.iter_batched(
            || input.clone().into_iter(),
            p01_sonarsweep::solve_iter,
            criterion::BatchSize::SmallInput,
        );
    });
    group.finish();
}
