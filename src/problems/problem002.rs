use super::utils::{fibonacci, is_even};

fn solve(limit: u128) -> u128 {
    let iterator = fibonacci();
    let result: u128 = iterator
        .filter(is_even)
        .take_while(|x: &u128| *x < limit)
        .sum();
    result
}

pub fn run() {
    const LIMIT: u128 = 4000000;
    let result2 = solve(LIMIT);
    println!("Solution of Problem 002 is : {result2}")
}

#[cfg(test)]
#[path = "tests/problem002.rs"]
mod test;
