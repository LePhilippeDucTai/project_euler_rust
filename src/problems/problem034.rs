use tailcall::tailcall;

fn factorial(n: u128) -> u128 {
    #[tailcall]
    fn factorial_rec(acc: u128, p: u128) -> u128 {
        if p == 0 {
            acc
        } else {
            factorial_rec(acc * p, p - 1)
        }
    }
    factorial_rec(1, n)
}

pub fn run() {
    let result = factorial(100);
    println!("{result}")
}
