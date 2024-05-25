// Special Pythagorean triplet
fn f(n: i32, a: i32, b: i32) -> i32 {
    n * n - 2 * n * (a + b) + 2 * a * b
}

fn find_pythagorean(a: i32, n: i32, lim_b: i32) -> Option<(i32, i32, i32)> {
    let range_b = a..lim_b;
    let values = range_b
        .map(|b| (a, b, f(n, a, b)))
        .filter(|t: &(i32, i32, i32)| t.2 == 0)
        .next();
    values
}

fn solve(n: i32) -> i32 {
    let lim_a = n / 3;
    let lim_b = 2 * n / 3;
    let result = (1..lim_a)
        .filter_map(|a| find_pythagorean(a, n, lim_b))
        .next();
    let (a, b, _) = result.expect("No solution found.");
    let c = n - a - b;
    return a * b * c;
}

pub fn run() {
    let result = solve(1000);
    println!("Solution of Problem 9 is : {result}")
}

#[cfg(test)]
#[path = "tests/problem009.rs"]
mod test;
