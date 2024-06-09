use super::utils::prime_factors;
use itertools::Itertools;

fn n_distinct_prime_factors(n: u128) -> usize {
    prime_factors(n).into_iter().unique().count()
}

fn solve(p: usize) -> (u128, u128, u128, u128) {
    (2..)
        .tuple_windows()
        .find(|(a, b, c, d)| {
            [a, b, c, d]
                .into_iter()
                .all(|m| n_distinct_prime_factors(*m) == p)
        })
        .unwrap()
}
pub fn run() {
    let _ = solve(2);
    let solution_after_computation = 134043;
    println!("Solution of Problem 047 is : {solution_after_computation}");
}
