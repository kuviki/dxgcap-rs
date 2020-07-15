use std::time::Instant;

use criterion::{Criterion, criterion_group, criterion_main};

use dxgcap::DXGIManager;
use winapi::_core::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("capture frame", |b| {
        b.iter_custom(|iters| {
            let mut elapsed = Duration::new(0, 0);
            for _ in 0..iters {
                let mut manager = DXGIManager::new(1000).unwrap();
                manager.capture_frame().unwrap();

                let start = Instant::now();
                manager.capture_frame().unwrap();
                elapsed += start.elapsed();
            }
            elapsed
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
