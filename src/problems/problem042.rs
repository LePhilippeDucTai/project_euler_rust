use std::collections::HashMap;

use itertools::Itertools;

use super::utils::{quadratic_solver, read_input_file};

fn alphabet_values() -> HashMap<char, u32> {
    // Create a new HashMap
    let mut alphabet_map: HashMap<char, u32> = HashMap::new();

    // Populate the HashMap with the alphabet and their positions
    for (i, c) in ('A'..='Z').enumerate() {
        alphabet_map.insert(c, (i + 1) as u32);
    }
    alphabet_map
}

fn preprocess_data(binding: String) -> Vec<String> {
    let data: Vec<String> = binding
        .lines()
        .flat_map(|s| {
            s.replace('\"', "")
                .split(',')
                .map(|s| s.to_string())
                .collect_vec()
        })
        .collect_vec();
    data
}

fn select_integer(x: f64) -> Option<i64> {
    if (x.fract() == 0.0) & (x > 0.0) {
        return Some(x as i64);
    }
    None
}

fn words_to_values(words: Vec<String>, dict: HashMap<char, u32>) -> Vec<u32> {
    let result = words
        .into_iter()
        .map(|s: String| s.chars().map(|c| dict.get(&c).unwrap()).sum::<u32>())
        .collect_vec();
    result
}
fn is_triangle(x: f64) -> bool {
    let a = 1.0;
    let b = 1.0;
    let c = -2.0 * x;

    if let Some((x1, x2)) = quadratic_solver(a, b, c) {
        match (select_integer(x1), select_integer(x2)) {
            (None, None) => false,
            _ => true,
        }
    } else {
        false
    }
}

fn solve(words: Vec<String>) -> usize {
    let dict = alphabet_values();

    words_to_values(words, dict)
        .into_iter()
        .filter(|x| is_triangle(*x as f64))
        .count()
}

pub fn run() {
    let filepath = "data/problems/problem042/0042_words.txt";
    let binding = read_input_file(filepath);
    let data = preprocess_data(binding);
    let solution = solve(data);
    println!("Solution of Problem 042 is : {solution}")
}
