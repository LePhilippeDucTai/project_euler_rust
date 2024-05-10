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

fn iter_primes(n: &u128) -> impl std::iter::Iterator<Item = u128> {
    (2..n + 1 as u128).filter(|x| is_prime(x))
}

fn prime_factors(n: u128) -> Vec<u128> {
    let collect_factors = |acc: Vec<u128>, prime: u128| {
        let mut p = n;
        let mut new_acc: Vec<u128> = acc.clone();
        while p % prime == 0 {
            new_acc.push(prime);
            p = p / prime;
        }
        return new_acc;
    };

    let v0: Vec<u128> = Vec::new();
    let factors = iter_primes(&n).fold(v0, collect_factors);
    return factors;
}

fn erastostene_sieve(n: u128) -> Vec<u128> {
    let mut acc: Vec<u128> = Vec::new();
    let m: usize = n as usize;
    let mut sieve: Vec<bool> = vec![true; m];
    for i in 0..m {
        if sieve[i] {
            let step = i + 2;
            let prime: u128 = step as u128;
            acc.push(prime);
            let range = ((2 * (step - 1))..m).step_by(step);
            for k in range {
                sieve[k] = false;
            }
        }
    }
    acc
}

pub fn run() {
    let n = 10;
    let result = prime_factors(n);
    println!("{result:?}");
    let m = 2000;
    let list_primes = erastostene_sieve(m);
    println!("{list_primes:?}");
    // let n = 10;
    // let xs: [bool; n] = [true; n];
    // println!("{xs:?}")
}

// pub fn run() {
//     let n_max = 10;
//     let ls_primes: Vec<i32> = (2..n_max).filter(is_prime).collect();
//     let prime_factors_prod: u128 = (2..n_max + 1 as u128)
//         .filter(|x: &u128| is_prime(x))
//         .product();
//     println!("{prime_factors_prod}");
// }
