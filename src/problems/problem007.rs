use super::utils::erastostene_sieve;

fn solve(n: u128) -> u128 {
    let mut m = n;
    let mut primes = erastostene_sieve(n);
    let lim = m as usize;
    while primes.len() < lim {
        m *= 2;
        primes = erastostene_sieve(m);
    }
    primes[n as usize - 1]
}

pub fn run() {
    let n = 10001;
    let result = solve(n);
    println!("Solution of Problem 007 is : {result}")
}

#[cfg(test)]
#[path = "tests/problem007.rs"]
mod test;
