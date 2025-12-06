use day6::{part1, part2};
use criterion::{criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../input/input.txt");

fn part1_bench(c: &mut Criterion) {
    c.bench_function("day6 part1", |b| {
        b.iter(|| {
            part1(&INPUT);
        });
    });
}

fn part2_bench(c: &mut Criterion) {
    c.bench_function("day6 part2", |b| {
        b.iter(|| {
            part2(&INPUT);
        });
    });
}

criterion_group!(benches, part1_bench, part2_bench);
criterion_main!(benches);
