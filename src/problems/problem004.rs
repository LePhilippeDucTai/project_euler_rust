use itertools::Itertools;

fn recursive_decompose(acc: Vec<i32>, p: i32, k: u32) -> Vec<i32> {
    if k == 0 {
        let mut new_acc = acc.clone();
        new_acc.push(p);
        new_acc
    } else {
        let tens: i32 = 10_i32.pow(k);
        let i = p / tens;
        let new_p = p - i * tens;
        let mut new_acc: Vec<i32> = acc.clone();
        new_acc.push(i);
        recursive_decompose(new_acc, new_p, k - 1)
    }
}

fn decompose(n: i32) -> Vec<i32> {
    let power: u32 = (n as f64).log10().floor() as u32;
    let empty = vec![];
    recursive_decompose(empty, n, power)
}

fn is_palindromic(n: i32) -> bool {
    let arr = decompose(n);
    let mut reversed = arr.clone();
    reversed.reverse();
    reversed == arr
}

fn solve(n_digits: u32) -> i32 {
    let tens = (10i32).pow(n_digits - 1);
    let limit = tens * 10;
    let ranges = tens..limit;
    let solution = ranges
        .combinations(2)
        .map(|x| x.iter().product())
        .filter(|n| is_palindromic(*n))
        .max()
        .unwrap();
    solution
}

pub fn run() {
    let solution = solve(3);
    println!("Solution of Problem 004 is : {solution}");
}

#[cfg(test)]
#[path = "tests/problem004.rs"]
mod test;
