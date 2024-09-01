use super::utils::pgcd_prime_cache;

fn phi(n: u32) -> usize {
    let range = 2..n;
    range
        .filter(|&x| pgcd_prime_cache(n as u64, x as u64) == 1)
        .count()
        + 1
}

fn totient(n: u32) -> f32 {
    (n as f32) / (phi(n) as f32)
}

fn select_max(acc: (u32, f32), x: (u32, f32)) -> (u32, f32) {
    let (_, f_acc) = acc;
    let (_, f) = x;
    if f_acc < f {
        x
    } else {
        acc
    }
}

pub fn run() {
    let ns = phi(7);
    println!("{ns:?}");
    let n = 1_000_00;
    let val = (2..=n)
        .step_by(2)
        // .par_bridge()
        .map(|x| (x, totient(x)))
        .reduce(select_max);
    println!("{val:?}")
}
