use aoc2021::{day_01, day_02, day_03};
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

criterion_main!(bench);
criterion_group!(
    name = bench;
    config = Criterion::default()
        .sample_size(1_000)
        .measurement_time(Duration::from_secs(10));
    targets =
        bench_01,
        bench_02,
        bench_03,
);

fn bench_01(c: &mut Criterion) {
    let input = day_01::parse_input_from_file("inputs/01.txt");
    let mut group = c.benchmark_group("Puzzle 01 - Sonar Sweep");
    group.bench_function("Part 1", |b| {
        b.iter(|| day_01::solve_part1(&input));
    });
    group.bench_function("Part 2", |b| {
        b.iter(|| day_01::solve_part2(&input));
    });
    group.finish();
}

fn bench_02(c: &mut Criterion) {
    let input = day_02::parse_input_from_file("inputs/02.txt");
    let mut group = c.benchmark_group("Puzzle 02 - Dive!");
    group.bench_function("Part 1", |b| {
        b.iter(|| day_02::solve_part1(&input));
    });
    group.bench_function("Part 2", |b| {
        b.iter(|| day_02::solve_part2(&input));
    });
    group.finish();
}

fn bench_03(c: &mut Criterion) {
    let input = day_03::parse_input_from_file("inputs/03.txt");
    let mut group = c.benchmark_group("Puzzle 03 - Binary Diagnostic");
    group.bench_function("Part 1", |b| {
        b.iter(|| day_03::solve_part1::<12>(&input));
    });
    group.bench_function("Part 2", |b| {
        b.iter(|| day_03::solve_part2::<12>(&input));
    });
    group.finish();
}
