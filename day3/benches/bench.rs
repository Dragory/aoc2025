use day3::{part1, part2};
use criterion::{criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../input/input.txt");

fn part1_bench(c: &mut Criterion) {
    c.bench_function("day3 part1", |b| {
        b.iter(|| {
            let lines: Vec<&str> = INPUT.lines().collect();
            part1(&lines);
        });
    });
}

fn part2_bench(c: &mut Criterion) {
    c.bench_function("day3 part2", |b| {
        b.iter(|| {
            let lines: Vec<&str> = INPUT.lines().collect();
            part2(&lines);
        });
    });
}

criterion_group!(benches, part1_bench, part2_bench);
criterion_main!(benches);
