fn totient(n: u64) -> u64 {
    let factors = primes::factors_uniq(n);
    let prod: f64 = factors
        .iter()
        .map(|&p| {
            let q = p as f64;
            1. - 1. / q
        })
        .product();
    ((n as f64) * prod) as u64
}

fn totient_ratio(n: u64) -> f64 {
    (n as f64) / (totient(n) as f64)
}

fn solve(limit: u64) -> (u64, f64) {
    let res = (2..=limit)
        .map(|x| (x, totient_ratio(x)))
        .max_by(|a, b| (a.1).partial_cmp(&b.1).unwrap())
        .unwrap();
    res
}

fn main() {
    let result = solve(1_000_000);
    println!("Solution of Problem 69 : {result:?}");
}
