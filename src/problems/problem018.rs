use std::iter::zip;

use itertools::Itertools;

use super::utils::read_input_file;

fn window_max(v: Vec<u32>) -> Vec<u32> {
    let w = v
        .windows(2)
        .map(|x| x.to_owned().into_iter().max().unwrap())
        .collect_vec();
    w
}

fn vec_add(v: Vec<u32>, w: Vec<u32>) -> Vec<u32> {
    let result = zip(v, w).map(|(a, b)| a + b).collect_vec();
    result
}

fn max_path_sum(last: Vec<u32>, next: Vec<u32>) -> Vec<u32> {
    let max_choices = window_max(last);
    let sum = vec_add(max_choices, next);
    return sum;
}

fn solve() {
    let input_file = read_input_file("data/problems/problem018/input.txt");
    let argument = input_file
        .lines()
        .map(|x| {
            x.split(' ')
                .map(|y| y.parse::<u32>().unwrap())
                .collect_vec()
        })
        .rev();
    let result = argument.reduce(max_path_sum).unwrap()[0];
    println!("Solution of Problem 18 is : {result}")
}

pub fn run() {
    solve()
}
