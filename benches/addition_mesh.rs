use criterion::{black_box, criterion_group, criterion_main, Criterion};
use parallel_addition_mesh::run_some_task;

use parallel_addition_mesh::data::prepared_data;

fn task_benchmarks(c: &mut Criterion) {
    c.bench_function("Sequential tasks", |f| {
        f.iter(|| run_some_task(prepared_data()))
    });
}

criterion_group!(benches, task_benchmarks);
criterion_main!(benches);
