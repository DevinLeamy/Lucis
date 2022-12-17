// rectangles_baseline

use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracer::*;

fn ray_trace() {
    let width = 800u32;
    let height = width;

    let (camera, scene) = Scene::rectangles();

    RayTracer::new(RayTracerConfig::default()).render_scene(&scene, camera, width, height);
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("Scene::rectangles", |b| b.iter(|| ray_trace()));
}

criterion_group!(
    name = ray_trace_benches;
    config = Criterion::default().sample_size(100);
    targets = benchmark
);
criterion_main!(ray_trace_benches);
