use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[cached::proc_macro::cached]
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn bench_fib(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

// fn bench_primes(c: &mut Criterion) {
//     c.bench_function("is_prime 41911311121991", |b| {
//         b.iter(|| is_prime(black_box(41911311121991)))
//     });
// }

criterion_group!(benches, bench_fib);
criterion_main!(benches);
