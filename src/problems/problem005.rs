fn is_prime(n: &u128) -> bool {
    let p = *n;
    if p == 2 {
        true
    } else {
        if p % 2 == 0 {
            return false;
        }
        let limit = (p as f32).sqrt() as u128;
        let any_divided: bool = (2..limit + 1).any(|x| (p % x) == 0);
        !any_divided
    }
}

fn iter_primes(n: &u128) -> impl std::iter::Iterator<Item = u128> {
    (2..n + 1 as u128).filter(|x| is_prime(x))
}

fn prime_factors(n: u128) -> Vec<u128> {
    let collect_factors = |acc: Vec<u128>, prime: u128| {
        let mut p = n;
        let mut new_acc: Vec<u128> = acc.clone();
        while p % prime == 0 {
            new_acc.push(prime);
            p = p / prime;
        }
        return new_acc;
    };

    let v0: Vec<u128> = Vec::new();
    let factors = iter_primes(&n).fold(v0, collect_factors);
    return factors;
}

fn divide_m_by(i: u128, v: &Vec<u128>) -> u128 {
    let mut m = i;
    for k in v {
        if m % k == 0 {
            m = m / k;
        }
    }
    m
}

fn solve(n: u128) -> u128 {
    let range = 2..n;
    let mut v = Vec::new();
    for i in range {
        if is_prime(&i) {
            v.push(i)
        } else {
            let m = divide_m_by(i, &v);
            let rest: Vec<u128> = prime_factors(m);
            v.extend(rest);
        }
    }
    v.into_iter().product()
}

pub fn run() {
    let n = 25;
    let problem = solve(n);
    println!("Solution of Problem 5 is : {problem:?}")
}

#[cfg(test)]
#[path = "tests/problem005.rs"]
mod test;
