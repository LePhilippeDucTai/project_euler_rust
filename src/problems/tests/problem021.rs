#[test]
fn test_solve() {
    let actual = super::solved(300);
    let expected = 504;
    assert_eq!(actual, expected);
}

#[test]
fn test_amicables() {
    let nums = super::amicable(220, 284);
    let expected = true;
    assert_eq!(nums, expected);
}
