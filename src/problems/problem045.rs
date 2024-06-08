fn triangle(n: &i64) -> i64 {
    n * (n + 1) / 2
}

fn quadratic_solver(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let delta = b.powi(2) - 4.0 * a * c;
    if delta < 0.0 {
        return None;
    }
    let (x1, x2) = (
        (-b - delta.sqrt()) / (2.0 * a),
        (-b + delta.sqrt()) / (2.0 * a),
    );

    Some((x1.min(x2), x1.max(x2)))
}

fn select_integer(x: f64, n: i64) -> Option<i64> {
    if x.fract() == 0.0 {
        if (0.0 < x) & (x < n as f64) {
            return Some(x as i64);
        } else {
            return None;
        }
    }
    None
}

fn find_solution(x1: f64, x2: f64, n: i64) -> Option<i64> {
    match (select_integer(x1, n), select_integer(x2, n)) {
        (None, None) => None,
        (Some(k), None) => Some(n - k),
        (None, Some(k)) => Some(n - k),
        (Some(k1), Some(k2)) => Some(n - k1.min(k2)),
    }
}
fn solve_pentagonal(n: i64) -> Option<i64> {
    let a: f64 = 3.0;
    let b = (1 - 6 * n) as f64;
    let c = (2 * n.pow(2) - 2 * n) as f64;
    let (x1, x2) = quadratic_solver(a, b, c).unwrap();
    find_solution(x1, x2, n)
}

fn solve_hexagonal(n: i64) -> Option<i64> {
    let a = 4.0;
    let b = (2 - 8 * n) as f64;
    let c = (3 * n.pow(2) - 3 * n) as f64;
    let (x1, x2) = quadratic_solver(a, b, c).unwrap();
    find_solution(x1, x2, n)
}

fn solve(from: i64) -> i64 {
    (from..)
        .find_map(|n| match (solve_pentagonal(n), solve_hexagonal(n)) {
            (None, None) => None,
            (None, Some(_)) => None,
            (Some(_), None) => None,
            _ => Some(triangle(&n)),
        })
        .unwrap()
}

pub fn run() {
    let from = 286;
    let result = solve(from);
    println!("Solution of Problem 045 is : {result}");
}
