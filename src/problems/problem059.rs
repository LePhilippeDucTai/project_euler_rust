use itertools::Itertools;

use crate::problems::utils::read_input_file;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
fn file_to_bytes(s: String) -> Vec<u8> {
    let res = s.split(",").map(|x| x.parse::<u8>().unwrap()).collect_vec();
    res
}

fn decrypt(data_bytes: Vec<u8>, key: Vec<&u8>) -> String {
    let result = data_bytes
        .into_iter()
        .tuple_windows()
        .flat_map(|(x, y, z)| {
            [x, y, z]
                .into_iter()
                .zip(&key)
                .map(|(u, &v)| (u ^ v).to_ascii_lowercase())
                .collect_vec()
        })
        .collect_vec();
    String::from_utf8(result).unwrap()
}

fn filter_words(text: &String) -> bool {
    text.contains("text")
}

fn solve() {
    let filepath = "data/problems/problem059/0059_cipher.txt";
    let binding = read_input_file(filepath);
    let data_bytes = file_to_bytes(binding);
    // println!("{data_bytes:?}");
    let all_combs = ALPHABET
        .as_bytes()
        .into_iter()
        .combinations(3)
        .map(|keys| decrypt(data_bytes.clone(), keys))
        .filter(filter_words)
        .collect_vec();
    println!("{all_combs:?}");
}

pub fn run() {
    solve()
}
