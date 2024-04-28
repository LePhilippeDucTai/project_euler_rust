fn prime_factors(n: u128) -> Vec<u128> {
    let mut p = n;
    let limit = (n as f64).sqrt() as u128;
    let mut k = 2;
    let mut result: Vec<u128> = vec![];
    while k < limit {
        if p % k == 0 {
            result.push(k);
            while p % k == 0 {
                p = p / k;
            }
        }
        k += 1;
    }
    return result;
}

fn solve(value: u128) -> u128 {
    let res: Vec<u128> = prime_factors(value);
    let max_value: &u128 = res.iter().max().unwrap();
    *max_value
}

pub fn run() {
    const VALUE: u128 = 600851475143;
    let result: u128 = solve(VALUE);
    println!("Solution of Problem 3 is : {:?}", result)
}

#[cfg(test)]
#[path = "tests/problem003.rs"]
mod test;
