use criterion::{criterion_group, criterion_main, Criterion};
use cuid2::*;

fn bench_create_id(c: &mut Criterion) {
    c.bench_function("generate cuid2", |b| b.iter(create_id));
}

fn bench_create_many_ids(c: &mut Criterion) {
    c.bench_function("generate many cuid2", |b| {
        b.iter(|| {
            (0..10_000).for_each(|_| {
                create_id();
            })
        })
    });
}

fn bench_create_small_id(c: &mut Criterion) {
    let constructor = CuidConstructor::new().with_length(10);
    c.bench_function("generate small cuid2", |b| {
        b.iter(|| constructor.create_id())
    });
}

criterion_group!(
    cuid2,
    bench_create_id,
    bench_create_many_ids,
    bench_create_small_id
);

criterion_main!(cuid2);
