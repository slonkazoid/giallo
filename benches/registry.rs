use criterion::{Criterion, criterion_group, criterion_main};
use giallo::Registry;

fn ser_benchmark(c: &mut Criterion) {
    let registry = Registry::builtin().unwrap();

    c.bench_function("registry serialization", |b| {
        b.iter(|| {
            let compressed = registry
                .dump_to_bytes(None)
                .expect("Failed to dump registry");
            std::hint::black_box(compressed);
        })
    });
}

fn deser_benchmark(c: &mut Criterion) {
    let dump = include_bytes!("../builtin.zst");

    c.bench_function("registry deserialization", |b| {
        b.iter(|| {
            let registry =
                Registry::load_from_compressed_bytes_bench(dump).expect("Failed to load registry");
            std::hint::black_box(registry);
        })
    });
}

criterion_group!(benches, deser_benchmark, ser_benchmark);
criterion_main!(benches);
