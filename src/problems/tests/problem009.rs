#[test]
fn test_solve() {
    let result = super::solve(12);
    let expected: i32 = 60;
    assert_eq!(result, expected)
}
