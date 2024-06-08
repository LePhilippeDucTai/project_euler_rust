use num_bigint::BigUint;

fn digits_sum(n: BigUint) -> u32 {
    n.to_string().chars().map(|x| x.to_digit(10).unwrap()).sum()
}

fn solve() {
    let base = BigUint::from(2u32);
    let exp = 1000u32;
    let x = base.pow(exp);
    let result = digits_sum(x);
    println!("Solution of Problem 016 is : {result}")
}

pub fn run() {
    solve()
}
