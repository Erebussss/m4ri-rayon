use criterion::{criterion_group, criterion_main, Criterion};

use m4ri_rayon::matrix::BM;

fn bench_binary_matrix_m4rm(c: &mut Criterion) {
    let size = 10000;
    let mut a = BM::new(size, size);
    let mut b = BM::new(size, size);
    a.rand();
    b.rand();
    let mut group = c.benchmark_group("binary_matrix_m4rm");
    group.sample_size(50);
    group.bench_function(&format!("{size}x{size}"), |x| x.iter(|| a.m4rm(&b)));
    group.finish();
}

criterion_group!(benches, bench_binary_matrix_m4rm);
criterion_main!(benches);
