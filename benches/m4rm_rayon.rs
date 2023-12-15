use criterion::{criterion_group, criterion_main, Criterion};

use m4ri_rayon::binary_matrix::BinaryMatrix;
use m4ri_rayon::m4rm;

fn bench_binary_matrix_m4rm(c: &mut Criterion) {
    let size = 10000;
    let mut a = BinaryMatrix::new(size, size);
    let mut b = BinaryMatrix::new(size, size);
    a.rand();
    b.rand();
    let mut group = c.benchmark_group("binary_matrix_m4rm");
    group.sample_size(10);
    group.bench_function(&format!("{size}x{size}"), |x| x.iter(|| m4rm(&a, &b)));
    group.finish();
}

criterion_group!(benches, bench_binary_matrix_m4rm);
criterion_main!(benches);
