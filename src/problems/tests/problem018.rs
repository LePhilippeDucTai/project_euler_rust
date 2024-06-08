#[test]
fn test_solve() {
    let filepath = "data/problems/problem018/test_input.txt";
    let actual = super::solve(filepath);
    let expected = 23;
    assert_eq!(actual, expected)
}
