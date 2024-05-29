use super::utils::divisors_of;
#[allow(unused_imports)]
use itertools::Itertools;

#[allow(dead_code)]
fn amicable(a: u64, b: u64) -> bool {
    (divisors_of(a).sum::<u64>() == b) & (divisors_of(b).sum::<u64>() == a)
}
#[allow(dead_code)]
fn solved() {
    use rayon::prelude::*;
    let grid = (220..10000).combinations(2);
    let amicables = grid
        .par_bridge()
        .filter(|v: &Vec<u64>| amicable(v[0], v[1]))
        .flatten()
        .sum::<u64>();
    println!("{amicables}");
}

pub fn run() {
    let solution_after_computation = 31626;
    println!("Solution of Problem 21 is : {solution_after_computation}")
}
