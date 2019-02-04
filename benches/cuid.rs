#[macro_use]
extern crate criterion;
extern crate cuid;

use criterion::Criterion;


fn bench_cuid(c: &mut Criterion) {
    c.bench_function(
        "generate cuid",
        |b| b.iter(|| cuid::cuid().unwrap())
    );
}


fn bench_slug(c: &mut Criterion) {
    c.bench_function(
        "generate cuid slug",
        |b| b.iter(|| cuid::slug().unwrap())
    );
}

criterion_group!(benches, bench_cuid, bench_slug);
criterion_main!(benches);
