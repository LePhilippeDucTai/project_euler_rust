// This is solved by obtaining the sum of the sequence : u(p) = 16p^2 + 4p + 4, for p = 1 to N
// This formula is obtained by the sum of the four terms in each corner.
// These corners are following the sequence : s(n, p) = 2p(n - 4(p-1)) + (2p-1)^2
// This is obtained by recurrence.
// The first four terms are generated by the sequence 2n + 1, for n = 1, ..., 4
// Then the next four terms are generated by 4n - 7 for n = 5, ..., 8
// Then the next sequence is generated by 6n - 23 for n = 9, ..., 12.
// By noticing at each spiral the sequence is arithmetic of factor 2p and first term equals to
// the last term which is always a odd number squared (2p - 1)^2.

fn solve(n: u64) -> u64 {
    let k = (n - 1) / 2;
    let p = k * (k + 1) / 2;
    let q = p * (2 * k + 1) / 3;
    4 * (4 * q + p + k) + 1
}

pub fn run() {
    let solution = solve(1001);
    println!("Solution of Problem 028 is : {solution}")
}