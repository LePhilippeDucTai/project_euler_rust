#[cfg(test)]
#[path = "tests/problem001.rs"]
mod test;

fn multiple_of_3_or_5(x: &i64) -> bool {
    (x % 5 == 0) | (x % 3 == 0)
}

fn solve(n_max: i64) -> i64 {
    let range = 1..n_max;
    let filtered = range.filter(multiple_of_3_or_5);
    filtered.sum()
}

pub fn run() {
    let result = solve(1000);
    println!("Solution of Problem 001 is : {result}");
}
