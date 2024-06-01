use itertools::Itertools;

#[test]
fn test_divisors_of() {
    let result = super::divisors_of(12).sorted().collect_vec();
    let expected = vec![1, 2, 3, 4, 6];
    assert_eq!(result, expected);

    let result = super::divisors_of(13).sorted().collect_vec();
    let expected = vec![1];
    assert_eq!(result, expected);

    let result = super::divisors_of(15).sorted().collect_vec();
    let expected = vec![1, 3, 5];
    assert_eq!(result, expected);

    let result = super::divisors_of(220).sorted().collect_vec();
    let expected = vec![1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110];
    assert_eq!(result, expected);
}
