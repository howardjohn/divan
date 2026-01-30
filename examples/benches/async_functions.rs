//! Run with:
//!
//! ```sh
//! cargo bench -q -p examples --bench async_functions --features async_tokio
//! ```

use divan::{black_box, Bencher};

fn main() {
    divan::main();
}

// Simple async function that returns a value
async fn async_add(a: u64, b: u64) -> u64 {
    a + b
}

// Async function performing computation
async fn async_compute(n: u64) -> u64 {
    // Simulate some computation
    let mut sum = 0;
    for i in 0..n {
        sum += i;
    }
    sum
}

#[divan::bench]
fn bench_async_simple(bencher: Bencher) {
    bencher.bench_async(|| async {
        black_box(async_add(black_box(1), black_box(2)).await)
    });
}

#[divan::bench]
fn bench_async_local_simple(bencher: Bencher) {
    bencher.bench_local_async(|| async {
        black_box(async_add(black_box(1), black_box(2)).await)
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn bench_async_with_args(bencher: Bencher, n: u64) {
    bencher
        .counter(n)
        .bench_async(|| async move {
            black_box(async_compute(black_box(n)).await)
        });
}

#[divan::bench]
fn bench_async_with_inputs(bencher: Bencher) {
    bencher
        .with_inputs(|| vec![1u64, 2, 3, 4, 5])
        .bench_async_values(|v| async move {
            black_box(v.iter().sum::<u64>())
        });
}

#[divan::bench]
fn bench_local_async_with_inputs(bencher: Bencher) {
    bencher
        .with_inputs(|| vec![1u64, 2, 3, 4, 5])
        .bench_local_async_values(|v| async move {
            black_box(v.iter().sum::<u64>())
        });
}
