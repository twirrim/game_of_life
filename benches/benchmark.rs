use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use gol::structs::Colony;
use gol::{initialise, process_frame};

const WIDTH: usize = 3840;
const HEIGHT: usize = 2160;

pub fn criterion_benchmark(c: &mut Criterion) {
    // Benchmark the init process
    c.bench_function("init 480000", |b| {
        b.iter(|| initialise(480_000, WIDTH, HEIGHT))
    });

    // Benchmark simple spinner
    let mut colony = Colony::new(WIDTH, HEIGHT);
    colony.make_alive(1, 2);
    colony.make_alive(2, 2);
    colony.make_alive(3, 2);
    colony.make_alive(11, 12);
    colony.make_alive(12, 12);
    colony.make_alive(13, 12);

    c.bench_with_input(BenchmarkId::new("small_spinner", 1), &colony, |b, s| {
        b.iter(|| process_frame(&mut s.clone()));
    });

    // Now try to make a much larger colony, but in a programmatic and consistent way.
    let mut colony = Colony::new(WIDTH, HEIGHT);
    for x in 0..800 {
        for y in 0..600 {
            colony.make_alive(x, y);
        }
    }
    c.bench_with_input(BenchmarkId::new("bigger_set", 1), &colony, |b, s| {
        b.iter(|| process_frame(&mut s.clone()));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
