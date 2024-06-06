use cached::proc_macro::cached;
use tailcall::tailcall;

#[cached]
fn factorial(n: u32) -> u128 {
    #[tailcall]
    fn factorial_rec(acc: u128, p: u32) -> u128 {
        if p == 0 {
            acc
        } else {
            factorial_rec(acc * (p as u128), p - 1)
        }
    }
    factorial_rec(1, n)
}

fn digits(n: u32) -> Vec<u32> {
    let mut dig = Vec::new();
    if n == 0 {
        dig.push(0);
        return dig;
    }
    let mut m = n;
    while m > 0 {
        dig.push(m % 10);
        m = m / 10;
    }
    dig.reverse();
    dig
}

fn is_curious(n: u32) -> bool {
    if n <= 2 {
        return false;
    }
    let sum_factorials: u32 = digits(n).into_iter().map(|x| factorial(x) as u32).sum();
    println!("{sum_factorials}");
    sum_factorials == n
}

fn solve() {}

pub fn run() {
    solve();
    let result = factorial(9);
    println!("{result}");
    let num = is_curious(145);
    println!("{num:?}")
}
