use super::utils::erastostene_sieve;

fn solve(n: u32) -> u128 {
    let primes = erastostene_sieve(n - 1);
    let result: u128 = primes.iter().map(|&x| x as u128).sum();
    result
}

pub fn run() {
    let result = solve(2_000_000);
    println!("Solution of Problem 10 is : {result}");
}
