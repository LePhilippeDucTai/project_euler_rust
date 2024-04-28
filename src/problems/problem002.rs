struct Fibonacci {
    curr: i128,
    next: i128,
}

impl Iterator for Fibonacci {
    type Item = i128;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

fn fibonacci(n: i16) -> i128 {
    fn aux(a: i128, b: i128, step: i16) -> i128 {
        if step < 0 {
            a
        } else {
            aux(b, a + b, step - 1)
        }
    }
    return aux(1, 1, n);
}

fn fibonacci_iterator() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn is_even(x: &i128) -> bool {
    x % 2 == 0
}

fn solve(limit: i128) -> i128 {
    let iterator: Fibonacci = fibonacci_iterator();
    let result: i128 = iterator
        .filter(is_even)
        .take_while(|x: &i128| *x < limit)
        .sum();
    result
}

pub fn run() {
    let _result: i128 = fibonacci(20);

    const LIMIT: i128 = 4000000;
    let result2 = solve(LIMIT);

    println!("Solution of Problem 2 is : {:?}", result2)
}

#[cfg(test)]
#[path = "tests/problem002.rs"]
mod test;
