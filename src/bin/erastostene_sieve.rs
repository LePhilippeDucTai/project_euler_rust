use itertools::Itertools;
use project_euler_rust::problems::utils::is_prime;

pub struct SpiralSequence {
    count: i128,
    p: i128,
    next: u128,
}

fn spiral(p: i128, k: i128) -> i128 {
    2 * p * (k + 1) + (2 * p - 1).pow(2)
}

impl Default for SpiralSequence {
    fn default() -> SpiralSequence {
        SpiralSequence {
            count: 0,
            p: 0,
            next: 1,
        }
    }
}

impl Iterator for SpiralSequence {
    type Item = u128;
    fn next(&mut self) -> Option<u128> {
        let result = Some(self.next);
        let k = self.count % 4;
        self.p += (k == 0) as i128;
        self.next = spiral(self.p, k) as u128;
        self.count += 1;
        result
    }
}

fn spiral_seq() -> SpiralSequence {
    SpiralSequence {
        ..Default::default()
    }
}

pub fn main() {
    let spiral_seq = spiral_seq();
    let test = spiral_seq
        .enumerate()
        .map(|(i, x)| (i, is_prime(x)))
        .scan(0, |acc, x| {
            let (i, y) = x;
            *acc += y as i32;
            let ratio = *acc as f32 / (i as f32);
            if ratio.is_nan() {
                return Some((i, 1.0));
            }
            Some((i, ratio))
        })
        .take_while(|(_, r)| (*r >= 0.1))
        .collect_vec();
    let (p, _) = test.last().unwrap();
    let solution = p / 2 + 1;
    println!("Solution of Problem 058 is : {solution}")
}
