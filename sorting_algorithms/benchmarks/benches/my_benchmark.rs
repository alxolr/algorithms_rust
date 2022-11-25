use criterion::{criterion_group, criterion_main, Criterion};

use heapsort::Heap;
use quicksort::quicksort;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting algoritms");

    group.bench_function("Heapsort", |b| {
        b.iter(|| {
            Heap::from(vec![1, 9, 11, 25, 36, 78, 55, 4, 36, 9, 87, 52, 3, 6]).into_sorted_vec()
        })
    });

    group.bench_function("Quicksort", |b| {
        b.iter(|| {
            let mut arr = vec![1, 9, 11, 25, 36, 78, 55, 4, 36, 9, 87, 52, 3, 6];

            quicksort(&mut arr);
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
