//! Checks whether it can be more beneficial to separate operating on each item and dropping it
//! when looping over a collection.
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[inline(never)]
fn prepare_only(data: Vec<String>) -> usize {
    data.len() * 13
}

#[inline(never)]
fn normal_drops(data: Vec<String>) -> usize {
    let mut total = 0;
    for item in data.into_iter() {
        total += item.len();
    }
    total
}

#[inline(never)]
fn delayed_drops(data: Vec<String>) -> usize {
    let mut total = 0;
    for item in data.iter() {
        total += item.len();
    }
    total
}

fn criterion_benchmark(c: &mut Criterion) {
    let data = vec![String::from("Hello, World!"); 33];

    c.bench_function("prepare_only", |b| {
        b.iter(|| prepare_only(black_box(data.clone())))
    });
    c.bench_function("normal_drops", |b| {
        b.iter(|| normal_drops(black_box(data.clone())))
    });
    c.bench_function("delayed_drops", |b| {
        b.iter(|| delayed_drops(black_box(data.clone())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
