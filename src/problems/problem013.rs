use super::utils::read_input_file;
use std::ops::Div;

fn first_characters(s: &str, n: usize) -> String {
    let last_n_chars = s.chars().take(n).collect::<String>();
    last_n_chars
}

pub fn run() {
    let file_path = "data/problems/problem013/input.txt";
    let binding = read_input_file(file_path);
    let base: u128 = 10;
    let k: u64 = 5;
    let exp = base.pow(k as u32 + 2);
    let input = binding
        .lines()
        .map(|x| {
            first_characters(x, 10 + k as usize)
                .parse::<u128>()
                .unwrap()
        })
        .sum::<u128>()
        .div(exp);
    println!("Solution of Problem 013 is : {input}")
}
