use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rustc_hash::FxHashSet;

use gol::structs::Cell;
use gol::{initialise, process_frame};

pub fn criterion_benchmark(c: &mut Criterion) {
    // Benchmark the init process
    c.bench_function("init 480000", |b| b.iter(|| initialise(480_000)));
    c.bench_function("init 4800000", |b| b.iter(|| initialise(4_800_000)));

    // Benchmark simple spinner
    let mut colony = FxHashSet::default();
    colony.insert(Cell { x: 1, y: 2 });
    colony.insert(Cell { x: 2, y: 2 });
    colony.insert(Cell { x: 3, y: 2 });

    colony.insert(Cell { x: 11, y: 12 });
    colony.insert(Cell { x: 12, y: 12 });
    colony.insert(Cell { x: 13, y: 12 });

    c.bench_with_input(BenchmarkId::new("small_spinner", 1), &colony, |b, s| {
        b.iter(|| process_frame(s));
    });

    // Now try to make a much larger colony, but in a programmatic and consistent way.

    let mut colony = FxHashSet::default();
    for x in 0..800 {
        for y in 0..600 {
            colony.insert(Cell { x, y });
        }
    }
    c.bench_with_input(BenchmarkId::new("bigger_set", 1), &colony, |b, s| {
        b.iter(|| process_frame(s));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
