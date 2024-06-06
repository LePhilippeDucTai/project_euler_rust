use std::fs;

fn convert_to_u8(x: char) -> u128 {
    x.to_string().parse().unwrap()
}

fn read_input_file_to_list(file_path: &str) -> Vec<u128> {
    fs::read_to_string(file_path)
        .expect("File input error.")
        .replace("\n", "")
        .chars()
        .map(convert_to_u8)
        .collect()
}

fn window_product(v: Vec<u128>, n: usize) -> Vec<u128> {
    v.windows(n)
        .into_iter()
        .map(|x| x.iter().product())
        .collect()
}

fn solve(numbers: Vec<u128>, n: usize) -> u128 {
    window_product(numbers, n).into_iter().max().unwrap()
}

fn read_input() -> Vec<u128> {
    let file_path = "data/problems/problem008/input.txt";
    let numbers = read_input_file_to_list(file_path);
    numbers
}
pub fn run() {
    let numbers = read_input();
    let result = solve(numbers, 13);
    println!("Solution of Problem 8 is : {result}");
}

#[cfg(test)]
#[path = "tests/problem008.rs"]
mod test;
