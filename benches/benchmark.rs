use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use gol::{initialise, process_frame, produce_neighbours};

pub fn criterion_benchmark(c: &mut Criterion) {
    // Benchmark the init process
    c.bench_function("init 480000", |b| b.iter(|| initialise(480_000)));

    // Benchmarking a large colony
    let mut colony = initialise(0);

    for x in 0..800 {
        for y in 0..600 {
            // Urgh...
            let neighbours = produce_neighbours(x as i32, y as i32);
            for (col_x, col_y) in neighbours {
                colony[col_x as usize][col_y as usize].neighbour_count += 1;
            }
            colony[x][y].alive = true;
        }
    }
    c.bench_with_input(BenchmarkId::new("bigger_set", 1), &colony, |b, s| {
        b.iter(|| process_frame(s));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
