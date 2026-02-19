use criterion::{Criterion, criterion_group, criterion_main};
use giallo::Registry;

fn registry_benchmark(c: &mut Criterion) {
    c.bench_function("registry deserialization", |b| {
        b.iter(|| {
            let registry =
                Registry::load_from_compressed_bytes_bench(include_bytes!("../builtin.msgpack"))
                    .expect("Failed to load registry");
            std::hint::black_box(registry);
        })
    });
}

criterion_group!(benches, registry_benchmark);
criterion_main!(benches);
