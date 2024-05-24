#[test]
fn test_solve() {
    let numbers = super::read_input();
    let result = super::solve(numbers, 4);
    let expected: u128 = 5832;
    assert_eq!(result, expected)
}
