pub fn erastostene_sieve(n: u32) -> Vec<u32> {
    let mut acc: Vec<u32> = Vec::new();
    let m: usize = n as usize;
    let mut sieve: Vec<bool> = vec![true; m];
    for i in 0..m {
        if sieve[i] {
            let step = i + 2;
            let prime: u32 = step as u32;
            acc.push(prime);
            let range = ((2 * (step - 1))..m).step_by(step);
            for k in range {
                sieve[k] = false;
            }
        }
    }
    acc
}

fn triangle_number(n: u32) -> u64 {
    (n * (n + 1) / 2) as u64
}

pub fn triangle_numbers(from: u32) -> impl Iterator<Item = u64> {
    (from..).map(|x| triangle_number(x))
}
