#[test]
fn test_solve() {
    let result: u128 = super::solve(10);
    let expected: u128 = 2520;
    assert_eq!(result, expected)
}
