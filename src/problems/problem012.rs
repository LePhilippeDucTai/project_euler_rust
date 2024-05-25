use super::utils::triangle_numbers;

fn n_divisors(n: u64) -> usize {
    (2..=n).filter(move |k| n % (*k) == 0).count() + 1
}

fn solve(n: u64, from: u32) -> u64 {
    let result = triangle_numbers(from)
        .filter(|v| n_divisors(*v) >= n as usize)
        .next()
        .expect("Triangle number not found.");
    result
}

pub fn run() {
    let res = solve(500, 12330);
    println!("Solution of Problem 12 is : {res}")
    // let result_after_computation = 76576500;
    // println!("Solution of Problem 12 is : {result_after_computation}")
}
