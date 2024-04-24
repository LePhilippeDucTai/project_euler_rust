fn fibonnaci(n: i16) -> i128 {
    fn aux(a: i128, b: i128, step: i16) -> i128 {
        if step < 0 {
            a
        } else {
            aux(b, a + b, step - 1)
        }
    }
    return aux(1, 1, n);
}

pub fn run() {
    let result = fibonnaci(100);
    println!("{}", result);
}
