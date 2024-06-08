#[test]
fn test_solve() {
    let from = 1;
    let actual = 40755;
    let expected = super::solve(from);
    assert_eq!(actual, expected);
}
