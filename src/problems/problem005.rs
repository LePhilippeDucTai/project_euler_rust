fn is_prime(n: &u128) -> bool {
    let p = *n;
    if p == 2 {
        true
    } else {
        if p % 2 == 0 {
            return false;
        }
        let limit = (p as f32).sqrt() as u128;
        let any_divided: bool = (2..limit + 1).any(|x| (p % x) == 0);
        !any_divided
    }
}

pub fn run() {
    let n_max = 10;
    // let ls_primes: Vec<i32> = (2..n_max).filter(is_prime).collect();
    let prime_factors_prod: u128 = (2..n_max + 1 as u128)
        .filter(|x: &u128| is_prime(x))
        .product();
    println!("{:?}", prime_factors_prod);
    // println!("{:?}", ls_primes)
}
