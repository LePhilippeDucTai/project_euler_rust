#[test]
fn test_solve() {
    let result: u32 = super::solve(10);
    let expected: u32 = 2640;
    assert_eq!(result, expected)
}
