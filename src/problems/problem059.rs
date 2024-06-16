use itertools::Itertools;

use crate::problems::utils::read_input_file;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn decrypt(binding: String, key: Vec<&u8>) -> Result<String, std::string::FromUtf8Error> {
    let input_file = binding
        .split(",")
        .map(|a| a.parse::<u8>().unwrap())
        .tuple_windows()
        .flat_map(|(x, y, z)| {
            [x, y, z]
                .into_iter()
                .zip((&key).into_iter())
                .map(|(u, &&v)| (u ^ v))
                .collect_vec()
        })
        .collect_vec();
    let text = String::from_utf8(input_file);
    text
}
fn solve() {
    let filepath = "data/problems/problem059/0059_cipher.txt";
    let binding = read_input_file(filepath);
    let all_combs = ALPHABET
        .as_bytes()
        .into_iter()
        .combinations(3)
        .map(|keys| decrypt(binding.clone(), keys))
        .collect_vec();
    println!("{all_combs:?}");
}

pub fn run() {
    solve()
}
