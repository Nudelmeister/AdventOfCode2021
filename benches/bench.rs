use aoc2021::day_01;
use criterion::{criterion_group, criterion_main, Criterion};

criterion_main!(bench);
criterion_group!(
    name = bench;
    config = Criterion::default().sample_size(1_000);
    targets = benchmark_01_sonarsweep,
);

fn benchmark_01_sonarsweep(c: &mut Criterion) {
    let input = day_01::read_input("inputs/01.txt").collect::<Vec<_>>();
    let mut group = c.benchmark_group("Puzzle 01 - Sonar Sweep");
    group.bench_function("Part 1", |b| {
        b.iter_batched(
            || input.clone().into_iter(),
            day_01::solve_part1,
            criterion::BatchSize::SmallInput,
        );
    });
    group.bench_function("Part 2", |b| {
        b.iter_batched(
            || input.clone().into_iter(),
            day_01::solve_part2,
            criterion::BatchSize::SmallInput,
        );
    });
    group.finish();
}
