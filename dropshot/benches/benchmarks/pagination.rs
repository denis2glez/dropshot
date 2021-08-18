use std::iter;

use criterion::{criterion_group, BenchmarkId, Criterion, Throughput};

fn basic_pagination(c: &mut Criterion) {
    // Setup (construct data, allocate memory, etc)
    static KB: usize = 1024;

    let mut group = c.benchmark_group("from_elem");
    for size in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &size| {
                b.iter(|| iter::repeat(0u8).take(size).collect::<Vec<_>>());
            },
        );
    }
    group.finish();
}

criterion_group!(benches, basic_pagination);
