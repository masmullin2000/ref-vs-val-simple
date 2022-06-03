use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ref_simple::{
    add_them, add_them2, add_them3, make_random_string, numbers_letters, numbers_letters_fold,
    numbers_letters_iter, numbers_letters_orig, numbers_letters_part,
};

const TIME: u64 = 12;

fn run_test(name: &str, str_size: usize, c: &mut Criterion) {
    //let mumble = "a1b2c3d4e5f6"; //make_random_string(255);
    let all_punc = make_random_string(str_size, 0, 0);
    let some_punc = make_random_string(str_size, 66, 0);
    let all_alpha = make_random_string(str_size, 100, 0);
    let all_num = make_random_string(str_size, 0, 100);
    let halvies = make_random_string(str_size, 50, 50);
    let fairs = make_random_string(str_size, 34, 33);
    macro_rules! make_bench {
        ($g:ident, $name:ident, $func:ident, $data:ident) => {
            $g.bench_function($name, |b| {
                b.iter(|| {
                    black_box($func(&$data));
                })
            });
        };
    }

    macro_rules! make_benches {
        ($g:ident, $name:literal, $func:ident) => {
            let name = $name.to_string();

            let all_punc_name = name.clone() + "_all_punc";
            make_bench!($g, all_punc_name, $func, all_punc);

            let some_punc_name = name.clone() + "_some_punc";
            make_bench!($g, some_punc_name, $func, some_punc);

            let all_alpha_name = name.clone() + "_all_alpha";
            make_bench!($g, all_alpha_name, $func, all_alpha);

            let all_num_name = name.clone() + "_all_num";
            make_bench!($g, all_num_name, $func, all_num);

            let halvies_name = name.clone() + "_halvies";
            make_bench!($g, halvies_name, $func, halvies);

            let fairs_name = name.clone() + "_fairs";
            make_bench!($g, fairs_name, $func, fairs);
        };
    }

    let mut g = c.benchmark_group(name);
    //g.measurement_time(std::time::Duration::from_secs(TIME));
    make_benches!(g, "opt", numbers_letters);
    make_benches!(g, "orig", numbers_letters_orig);
    make_benches!(g, "iter", numbers_letters_iter);
    make_benches!(g, "part", numbers_letters_part);
    make_benches!(g, "fold", numbers_letters_fold);
    /*g.bench_function("num_let_orig", |b| {*/
    /*b.iter(|| {*/
    /*black_box(numbers_letters_orig(&no_punc));*/
    /*})*/
    /*});*/
    /*g.bench_function("num_let_iter", |b| {*/
    /*b.iter(|| {*/
    /*black_box(numbers_letters_iter(&no_punc));*/
    /*})*/
    /*});*/
    /*g.bench_function("num_let_part", |b| {*/
    /*b.iter(|| {*/
    /*black_box(numbers_letters_part(&no_punc));*/
    /*})*/
    /*});*/
    /*g.bench_function("num_let_fold", |b| {*/
    /*b.iter(|| {*/
    /*black_box(numbers_letters_fold(&no_punc));*/
    /*})*/
    /*});*/

    /*g.bench_function("num_let_punc", |b| {*/
    /*b.iter(|| {*/
    /*black_box(numbers_letters(&with_punc));*/
    /*})*/
    /*});*/
    /*g.bench_function("num_let_orig_punc", |b| {*/
    /*b.iter(|| {*/
    /*black_box(numbers_letters_orig(&with_punc));*/
    /*})*/
    /*});*/
    /*g.bench_function("num_let_iter_punc", |b| {*/
    /*b.iter(|| {*/
    /*black_box(numbers_letters_iter(&with_punc));*/
    /*})*/
    /*});*/
    /*g.bench_function("num_let_part_punc", |b| {*/
    /*b.iter(|| {*/
    /*black_box(numbers_letters_part(&with_punc));*/
    /*})*/
    /*});*/
    g.finish();
}

fn bench_tiny(c: &mut Criterion) {
    run_test("tiny", 8, c);
}

fn bench_medium(c: &mut Criterion) {
    run_test("medium", 255, c);
}

fn bench_long(c: &mut Criterion) {
    run_test("long", 4096, c);
}

fn bench_signal(c: &mut Criterion) {
    let signal7 = match std::fs::read_to_string("signal.7.html") {
        Ok(s) => s,
        _ => "signal.7.html".to_string(),
    };

    macro_rules! make_bench {
        ($g:ident, $name:literal, $func:ident, $data:ident) => {
            $g.bench_function($name, |b| {
                b.iter(|| {
                    black_box($func(&$data));
                })
            });
        };
    }

    let mut g = c.benchmark_group("signal");
    g.measurement_time(std::time::Duration::from_secs(TIME));

    make_bench!(g, "opt_signal", numbers_letters, signal7);
    make_bench!(g, "orig_signal", numbers_letters_orig, signal7);
    make_bench!(g, "iter_signal", numbers_letters_iter, signal7);
    make_bench!(g, "part_signal", numbers_letters_part, signal7);
    make_bench!(g, "fold_signal", numbers_letters_fold, signal7);

    g.finish();
}

criterion_group!(benches, bench_signal, bench_tiny, bench_medium, bench_long);
criterion_main!(benches);
