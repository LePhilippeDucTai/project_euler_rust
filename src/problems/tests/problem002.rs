#[test]
fn test_solve() {
    let result: u128 = super::solve(89);
    let expected: u128 = 2 + 8 + 34;
    assert_eq!(result, expected)
}
