use super::utils::{divisors_of, triangle_numbers};

fn n_divisors(n: u64) -> usize {
    divisors_of(n).count() + 1
}

fn solve(n: u64, from: u64) -> u64 {
    
    triangle_numbers(from).find(|v| n_divisors(*v) >= n as usize)
        .expect("Triangle number not found.")
}

pub fn run() {
    let res = solve(500, 1);
    println!("Solution of Problem 12 is : {res}")
}
