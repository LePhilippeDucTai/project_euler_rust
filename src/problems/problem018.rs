use std::iter::zip;

use itertools::Itertools;

use super::utils::read_input_file;

fn prepare_input(input_file: &str) -> Vec<Vec<u32>> {
    let argument: Vec<Vec<u32>> = input_file
        .lines()
        .map(|x| {
            x.split(' ')
                .map(|y| y.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .rev()
        .collect();
    argument
}
fn window_max(v: Vec<u32>) -> Vec<u32> {
    v.windows(2)
        .map(|x| x.iter().copied().max().unwrap())
        .collect_vec()
}

fn vec_add<T: std::ops::Add>(v: Vec<T>, w: Vec<T>) -> Vec<<T as std::ops::Add>::Output> {
    zip(v, w).map(|(a, b)| a + b).collect_vec()
}

fn max_path_sum(last: Vec<u32>, next: Vec<u32>) -> Vec<u32> {
    let max_choices = window_max(last);
    vec_add(max_choices, next)
}

pub fn solve(filepath: &str) -> u32 {
    let input_file = read_input_file(filepath);
    let pyramid = prepare_input(input_file.as_str());

    pyramid.into_iter().reduce(max_path_sum).unwrap()[0]
}

pub fn run() {
    let filepath = "data/problems/problem018/input.txt";
    let result = solve(filepath);
    println!("Solution of Problem 018 is : {result}")
}

#[cfg(test)]
#[path = "tests/problem018.rs"]
mod test;
