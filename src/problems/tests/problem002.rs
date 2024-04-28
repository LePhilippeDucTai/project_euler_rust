#[test]
fn test_solve() {
    let result = super::solve(89);
    const EXPECTED: i128 = 2 + 8 + 34;
    assert_eq!(result, EXPECTED)
}
