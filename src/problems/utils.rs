use itertools::Itertools;
use std::cmp::PartialEq;
use std::ops::Rem;
use tailcall::tailcall;

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

pub fn is_even<T>(n: &T) -> bool
where
    T: Rem<Output = T> + PartialEq + From<u8> + Copy,
{
    (*n) % T::from(2) == T::from(0)
}

pub fn is_prime(n: u128) -> bool {
    let p = n;
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
    (2..n + 1 as u128).filter(|x| is_prime(*x))
}

pub fn prime_factors(n: u128) -> Vec<u128> {
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
    let limit = (n as f32).sqrt() as u64;
    let divisors = (1..=limit).filter(move |k| n % (*k) == 0);
    let more_divisors = divisors.clone().filter(|x| *x != 1).map(move |x| n / x);
    divisors.chain(more_divisors).unique()
}

fn triangle_number(n: u64) -> u64 {
    n * (n + 1) / 2
}

pub fn triangle_numbers(from: u64) -> impl Iterator<Item = u64> {
    (from..).map(|x| triangle_number(x))
}

pub struct Fibonacci {
    curr: u128,
    next: u128,
}

fn fibo(acc: u128, x: u128) -> (u128, u128) {
    return ((acc + x), acc);
}
impl Iterator for Fibonacci {
    type Item = u128;

    fn next(&mut self) -> Option<u128> {
        (self.next, self.curr) = fibo(self.next, self.curr);
        let current = self.curr;
        Some(current)
    }
}

pub fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

#[tailcall]
pub fn pgcd(a: u64, b: u64) -> u64 {
    let result = if b == 0 { a } else { pgcd(b, a % b) };
    result
}

#[cfg(test)]
#[path = "tests/utils.rs"]
mod test;
