use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ref_simple::{add_them, add_them2, add_them3};

fn run_test(name: &str, time: u8, c: &mut Criterion) {
    let mut g = c.benchmark_group(name);
    g.measurement_time(std::time::Duration::from_secs(time as u64));
    g.bench_function("add_them", |b| {
        b.iter(|| {
            let x = 1;
            let y = 2;
            black_box(add_them(&x, &y))
        })
    });
    g.bench_function("add_them2", |b| {
        b.iter(|| {
            let x = 1;
            let y = 2;
            black_box(add_them2(x, y));
        })
    });
    g.bench_function("add_them3", |b| {
        b.iter(|| {
            let x = 1;
            let y = 2;
            black_box(add_them3(x, &y));
        })
    });
    //g.bench_function("unstable sort", |b| b.iter(|| black_box(ans_unst.sort_unstable())));
    g.finish();
}

fn bench_tiny(c: &mut Criterion) {
    run_test("tiny", 10, c);
}

fn bench_medium(c: &mut Criterion) {
    run_test("medium", 30, c);
}

fn bench_long(c: &mut Criterion) {
    run_test("long", 50, c);
}

criterion_group!(benches, bench_tiny, bench_medium, bench_long);
criterion_main!(benches);
