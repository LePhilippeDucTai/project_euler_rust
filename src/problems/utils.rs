pub fn erastostene_sieve(n: u32) -> Vec<u32> {
    let mut acc: Vec<u32> = Vec::new();
    let m: usize = n as usize;
    let mut sieve: Vec<bool> = vec![true; m];
    for i in 0..m {
        if sieve[i] {
            let step = i + 2;
            let prime: u32 = step as u32;
            acc.push(prime);
            let range = ((2 * (step - 1))..m).step_by(step);
            for k in range {
                sieve[k] = false;
            }
        }
    }
    acc
}
#[allow(dead_code)]
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
#[allow(dead_code)]
fn iter_primes(n: &u128) -> impl std::iter::Iterator<Item = u128> {
    (2..n + 1 as u128).filter(|x| is_prime(x))
}
#[allow(dead_code)]
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

pub fn divisors_of(n: u64) -> impl Iterator<Item = u64> {
    (1..=(n / 2)).filter(move |k| n % (*k) == 0)
}

fn triangle_number(n: u32) -> u64 {
    (n * (n + 1) / 2) as u64
}

pub fn triangle_numbers(from: u32) -> impl Iterator<Item = u64> {
    (from..).map(|x| triangle_number(x))
}
