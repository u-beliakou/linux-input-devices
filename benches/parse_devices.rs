use criterion::{criterion_group, criterion_main, Criterion};

use linux_input_devices::parse_devices;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_devices", |b| b.iter(|| parse_devices("./assets/example.txt")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);