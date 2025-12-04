use day4::{part1, part2, part1_set, part2_set};
use criterion::{criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../input/input.txt");

fn part1_bench(c: &mut Criterion) {
    c.bench_function("day4 part1", |b| {
        b.iter(|| {
            let diagram = day4::parse_diagram(INPUT);
            part1(&diagram);
        });
    });
}

fn part2_bench(c: &mut Criterion) {
    c.bench_function("day4 part2", |b| {
        b.iter(|| {
            let diagram = day4::parse_diagram(INPUT);
            part2(&diagram);
        });
    });
}

fn part1_set_bench(c: &mut Criterion) {
    c.bench_function("day4 part1_set", |b| {
        b.iter(|| {
            part1_set(INPUT);
        });
    });
}

fn part2_set_bench(c: &mut Criterion) {
    c.bench_function("day4 part2_set", |b| {
        b.iter(|| {
            part2_set(INPUT);
        });
    });
}

criterion_group!(benches, part1_bench, part2_bench, part1_set_bench, part2_set_bench);
criterion_main!(benches);
