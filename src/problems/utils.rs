#[allow(dead_code)]
fn erastostene_sieve(n: u128) -> Vec<u128> {
    let mut acc: Vec<u128> = Vec::new();
    let m: usize = n as usize;
    let mut sieve: Vec<bool> = vec![true; m];
    for i in 0..m {
        if sieve[i] {
            let step = i + 2;
            let prime: u128 = step as u128;
            acc.push(prime);
            let range = ((2 * (step - 1))..m).step_by(step);
            for k in range {
                sieve[k] = false;
            }
        }
    }
    acc
}
