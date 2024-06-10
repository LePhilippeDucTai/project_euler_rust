use itertools::Itertools;

use super::utils::read_input_file;

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
pub fn run() {
    let filepath = "data/problems/problem042/0042_words.txt";
    let binding = read_input_file(filepath);
    let data = preprocess_data(binding);
    println!("{data:?}")
}
