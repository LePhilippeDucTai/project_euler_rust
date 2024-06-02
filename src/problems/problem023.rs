use super::utils::divisors_of;
use super::utils::triangle_number;
use itertools::Itertools;

const LIMIT: u64 = 28123;

fn is_abundant(n: u64) -> bool {
    let divisors_sum = divisors_of(n as u64).sum::<u64>();
    divisors_sum > n
}

fn abundant_numbers() -> Vec<u64> {
    (1..=LIMIT).filter(|&x| is_abundant(x)).collect_vec()
}

fn solve() {
    let sum_of_two_abundants = abundant_numbers()
        .into_iter()
        .combinations_with_replacement(2)
        .filter(|v| (v[0] <= LIMIT) & (v[1] <= LIMIT))
        .map(|x| x.into_iter().sum::<u64>())
        .filter(|x| (*x) <= LIMIT)
        .unique()
        .sum::<u64>();
    let sum_limit = triangle_number(LIMIT);
    let result = sum_limit - sum_of_two_abundants;
    println!("Solution of Problem 23 is : {result}")
}

pub fn run() {
    solve()
}
