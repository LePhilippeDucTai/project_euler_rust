fn recursive_decompose(acc: Vec<i32>, p: i32, k: u32) -> Vec<i32> {
    if k == 0 {
        let mut new_acc = acc.clone();
        new_acc.push(p);
        new_acc
    } else {
        let tens: i32 = (10 as i32).pow(k);
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

fn is_palindromic(arr: Vec<i32>) -> bool {
    let mut reversed = arr.clone();
    reversed.reverse();
    return reversed == arr;
}

fn solve(n_digits: u32) -> i32 {
    let tens = (10 as i32).pow(n_digits - 1);
    let limit = tens * 10 - 1;
    let ranges = tens..limit;
    0
}

pub fn run() {
    let n = 10133101;
    let result = decompose(n);
    let is_palind = is_palindromic(result);
    println!("{:?}", is_palind);
}
