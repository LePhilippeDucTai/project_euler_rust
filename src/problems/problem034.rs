use super::utils::factorial;

fn digits(n: u32) -> Vec<u32> {
    let mut dig = Vec::new();
    if n == 0 {
        dig.push(0);
        return dig;
    }
    let mut m = n;
    while m > 0 {
        dig.push(m % 10);
        m /= 10;
    }
    dig.reverse();
    dig
}

fn is_curious(n: u32) -> bool {
    if n <= 2 {
        return false;
    }
    let sum_factorials: u32 = digits(n).into_iter().map(|x| factorial(x) as u32).sum();
    sum_factorials == n
}

fn solve() {}

pub fn run() {
    solve();
    let _result = factorial(9);
    // println!("{result}");
    let _num: bool = is_curious(145);
    // println!("{num:?}");
}
