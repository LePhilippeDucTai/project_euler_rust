#[test]
fn test_solve() {
    let result: i128 = super::solve(89);
    let expected: i128 = 2 + 8 + 34;
    assert_eq!(result, expected)
}
