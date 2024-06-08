fn pgcd(a: u64, b: u64) -> u64 {
    if a < b {
        return pgcd(b, a);
    }
    if b == 0 {
        return a;
    }
    pgcd(b, a % b)
}

fn ppcm(a: u64, b: u64) -> u64 {
    a * b / pgcd(a, b)
}

fn solve(n: u64) -> u64 {
    (1..=n).fold(1, ppcm)
}

pub fn run() {
    let n = 20;
    let problem = solve(n);
    println!("Solution of Problem 005 is : {problem:?}")
}

#[cfg(test)]
#[path = "tests/problem005.rs"]
mod test;
