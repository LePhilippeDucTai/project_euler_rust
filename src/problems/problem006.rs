fn integer_sum(n: u32) -> u32 {
    n * (n + 1) / 2
}

fn integer_squared_sum(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

fn solve(n: u32) -> u32 {
    let s = integer_sum(n);
    let s2 = integer_squared_sum(n);
    u32::pow(s, 2) - s2
}

pub fn run() {
    let n = 100;
    let result = solve(n);
    println!("Solution of Problem 6 is : {result}")
}

#[cfg(test)]
#[path = "tests/problem006.rs"]
mod test;
