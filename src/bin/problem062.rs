fn is_cube(x: u64) -> bool {
    if x == 0 {
        return true;
    }
    let mut low = 0;
    let mut high = x;
    while low <= high {
        let mid = (low + high) / 2;
        let val = mid * mid * mid;
        if x == val {
            return true;
        } else if x < val {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    false
}

fn main() {
    let x: u64 = 81;
    let res = is_cube(x);
    assert_eq!(res, false); // 81 n'est pas un cube parfait
}

#[test]
fn test_main() {
    assert_eq!(is_cube(27), true);
    assert_eq!(is_cube(64), true);
    assert_eq!(is_cube(81), false); // 81 n'est pas un cube parfait
}
