use std::collections::VecDeque;

use criterion::{criterion_group, criterion_main, Criterion};
use queue::Queue;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Queue vs VecDeque");
    let size = 100_000;
    let arr = vec![100; size];

    group.bench_function("Queue", |b| {
        let mut queue = Queue::new(size);
        b.iter(|| {
            arr.clone()
                .into_iter()
                .for_each(|element| queue.push(element));

            for _ in 0..arr.len() {
                queue.pop();
            }
        })
    });

    group.bench_function("VecDeque", |b| {
        let mut queue = VecDeque::new();

        b.iter(|| {
            arr.clone()
                .into_iter()
                .for_each(|element| queue.push_front(element));

            for _ in 0..arr.len() {
                queue.pop_front();
            }
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
