use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use gol::structs::Colony;
use gol::{initialise, process_frame};

const WIDTH: isize = 3840;
const HEIGHT: isize = 2160;

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

    // Using a neat "gun" pattern, small but can be looped many times.
    // This should hopefully help figure out performance over time.
    let mut colony = Colony::new(1920, 1080);
    for cell in [
        (0, 24),
        (1, 22),
        (1, 24),
        (2, 12),
        (2, 13),
        (2, 20),
        (2, 21),
        (2, 34),
        (2, 35),
        (3, 11),
        (3, 15),
        (3, 20),
        (3, 21),
        (3, 34),
        (3, 35),
        (4, 0),
        (4, 1),
        (4, 10),
        (4, 16),
        (4, 20),
        (4, 21),
        (5, 0),
        (5, 1),
        (5, 10),
        (5, 14),
        (5, 16),
        (5, 17),
        (5, 22),
        (5, 24),
        (6, 10),
        (6, 16),
        (6, 24),
        (7, 11),
        (7, 15),
        (8, 12),
        (8, 13),
    ] {
        colony.make_alive(cell.0, cell.1);
    }
    c.bench_with_input(BenchmarkId::new("gospel_glider", 1), &colony, |b, s| {
        b.iter(|| {
            for _ in 0..10 {
                process_frame(&mut s.clone())
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
