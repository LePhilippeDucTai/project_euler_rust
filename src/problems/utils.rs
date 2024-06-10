use cached::proc_macro::cached;
use itertools::Itertools;
use std::cmp::PartialEq;
use std::fs;
use std::ops::Rem;
use tailcall::tailcall;

#[cached]
pub fn erastostene_sieve(n: u128) -> Vec<u128> {
    let mut acc: Vec<u128> = Vec::new();
    let m: usize = n as usize;
    let mut sieve: Vec<bool> = vec![true; m];
    for i in 0..(m - 1) {
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

pub fn is_even<T>(n: &T) -> bool
where
    T: Rem<Output = T> + PartialEq + From<u8> + Copy,
{
    (*n) % T::from(2) == T::from(0)
}

#[cached]
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

pub fn prime_factors(n: u128) -> Vec<u128> {
    let collect_factors = |acc: Vec<u128>, prime: u128| {
        let mut p = n;
        let mut new_acc: Vec<u128> = acc.clone();
        while p % prime == 0 {
            new_acc.push(prime);
            p /= prime;
        }
        new_acc
    };
    let v0: Vec<u128> = Vec::new();
    erastostene_sieve(n).into_iter().fold(v0, collect_factors)
}

pub fn divisors_of(n: u64) -> impl Iterator<Item = u64> {
    let limit = (n as f32).sqrt() as u64;
    let divisors = (1..=limit).filter(move |k| n % (*k) == 0);
    let more_divisors = divisors.clone().filter(|x| *x != 1).map(move |x| n / x);
    divisors.chain(more_divisors).unique()
}

pub fn triangle(n: &u64) -> u64 {
    n * (n + 1) / 2
}

pub fn triangle_numbers(from: u64) -> impl Iterator<Item = u64> {
    (from..).map(|x| triangle(&x))
}

pub struct Fibonacci {
    curr: u128,
    next: u128,
}

fn fibo(acc: u128, x: u128) -> (u128, u128) {
    ((acc + x), acc)
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
    if b == 0 {
        a
    } else {
        pgcd(b, a % b)
    }
}

#[cached]
pub fn factorial(n: u32) -> u128 {
    #[tailcall]
    fn factorial_rec(acc: u128, p: u32) -> u128 {
        if p == 0 {
            acc
        } else {
            factorial_rec(acc * (p as u128), p - 1)
        }
    }
    factorial_rec(1, n)
}
fn merge_sorted(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut _left = left.iter();
    let mut _right = right.iter();
    match (_left.next(), _right.next()) {
        (None, None) => vec![],
        (Some(&x), None) | (None, Some(&x)) => vec![x],
        (Some(&x), Some(&y)) => {
            let mut result;
            if x < y {
                result = vec![x];
                result.extend(merge_sorted(left[1..].to_vec(), right));
            } else {
                result = vec![y];
                result.extend(merge_sorted(left, right[1..].to_vec()));
            };
            result
        }
    }
}

pub fn mergesort(v: &mut [i32]) -> Vec<i32> {
    match v {
        [_] | [] => v.to_vec(),
        [x, y] => {
            if x > y {
                v.swap(0, 1);
            }
            v.to_vec()
        }
        _ => {
            let mid = v.len() / 2;
            let (left, right) = v.split_at_mut(mid);
            merge_sorted(mergesort(left), mergesort(right))
        }
    }
}

pub fn merge_sorted_2(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let (n, m) = (left.len(), right.len());
    let (mut i, mut j) = (0, 0);
    let (mut a, mut b);
    while (i < n) & (j < m) {
        (a, b) = (left[i], right[j]);
        if a < b {
            result.push(a);
            i += 1;
        } else {
            result.push(b);
            j += 1;
        }
    }
    if (i == n) & (j < m) {
        result.append(&mut right.as_slice()[j..].to_vec());
    } else {
        result.append(&mut left.as_slice()[i..].to_vec());
    }
    result
}

pub fn n_digits(n: u128) -> u32 {
    (f64::log10(n as f64) + 1.0) as u32
}

pub fn read_input_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("File input error.")
}

pub fn quadratic_solver(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let delta = b.powi(2) - 4.0 * a * c;
    if delta < 0.0 {
        return None;
    }
    let (x1, x2) = (
        (-b - delta.sqrt()) / (2.0 * a),
        (-b + delta.sqrt()) / (2.0 * a),
    );

    Some((x1.min(x2), x1.max(x2)))
}

#[cfg(test)]
#[path = "tests/utils.rs"]
mod test;
