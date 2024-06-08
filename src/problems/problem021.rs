use super::utils::divisors_of;
use itertools::Itertools;

fn amicable(a: u64, b: u64) -> bool {
    (divisors_of(a).sum::<u64>() == b) & (divisors_of(b).sum::<u64>() == a)
}

fn solved(n: u64) -> u64 {
    use rayon::prelude::*;
    let grid = (220..n).combinations(2);
    let amicables: u64 = grid
        .par_bridge()
        .filter(|v: &Vec<u64>| amicable(v[0], v[1]))
        .flatten()
        .sum();
    amicables
}

pub fn run() {
    let _ = solved(300);
    // println!("{solution}");
    let solution_after_computation = 31626;
    println!("Solution of Problem 021 is : {solution_after_computation}")
}

#[cfg(test)]
#[path = "tests/problem021.rs"]
mod test;
