fn collatz(acc: &mut u64, _: u64) -> Option<u64> {
    if *acc % 2 == 0 {
        *acc = *acc / 2;
    } else {
        *acc = 3 * *acc + 1
    }
    Some(*acc)
}

fn collatz_sequence(start: u64) -> impl Iterator<Item = u64> {
    (1..).scan(start, collatz)
}

fn collatz_sequence_length(start: u64) -> usize {
    collatz_sequence(start).take_while(|x| *x != 1).count() + 2
}

fn solve(n: u64, start: u64) -> u64 {
    let starting_number = start..=n;
    let res: Option<(u64, usize)> = starting_number
        .map(|x| (x, collatz_sequence_length(x)))
        .max_by_key(|x| x.1);
    let result = res.unwrap();
    result.0
}

pub fn run() {
    let n = 1_000_000;
    let result = solve(n, 800_000);
    println!("Solution of Problem 14 is : {result}")
}
