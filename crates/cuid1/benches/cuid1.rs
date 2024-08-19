use std::thread;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_cuid(c: &mut Criterion) {
    c.bench_function("generate cuid", |b| b.iter(cuid1::cuid));
}

fn bench_many_cuids(c: &mut Criterion) {
    c.bench_function("generate many cuids", |b| {
        b.iter(|| {
            (0..10_000).for_each(|_| {
                cuid1::cuid();
            })
        })
    });
}

fn bench_slug(c: &mut Criterion) {
    c.bench_function("generate cuid slug", |b| b.iter(cuid1::slug));
}

fn bench_many_slugs(c: &mut Criterion) {
    c.bench_function("generate many slugs", |b| {
        b.iter(|| {
            (0..10_000).for_each(|_| {
                cuid1::slug();
            })
        })
    });
}

/// Generate a bunch of UUIDs across 4 threads.
fn bench_multithread_perf(c: &mut Criterion) {
    c.bench_function("multithreaded perf", |b| {
        b.iter(|| {
            let mut threads: Vec<thread::JoinHandle<_>> = vec![];

            for _ in 0..4 {
                let t = thread::spawn(move || {
                    for _ in 0..10_000 {
                        cuid1::cuid();
                    }
                });
                threads.push(t);
            }

            threads.into_iter().for_each(|t| {
                t.join().unwrap();
            });
        })
    });
}

/// Generate a bunch of UUIDs across 4 threads.
fn bench_multithread_perf_lots_of_threads(c: &mut Criterion) {
    c.bench_function("multithreaded perf (many threads)", |b| {
        b.iter(|| {
            let mut threads: Vec<thread::JoinHandle<_>> = vec![];

            for _ in 0..16 {
                let t = thread::spawn(move || {
                    for _ in 0..10_000 {
                        cuid1::cuid();
                    }
                });
                threads.push(t);
            }

            threads.into_iter().for_each(|t| {
                t.join().unwrap();
            });
        })
    });
}

criterion_group!(
    benches,
    bench_cuid,
    bench_slug,
    bench_many_cuids,
    bench_many_slugs,
    bench_multithread_perf,
    bench_multithread_perf_lots_of_threads
);
criterion_main!(benches);
