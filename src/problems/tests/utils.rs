use itertools::Itertools;

#[test]
fn test_divisors_of() {
    let result = super::divisors_of(12).sorted().collect_vec();
    let expected = vec![1, 2, 3, 4, 6, 12];
    assert_eq!(result, expected);

    let result = super::divisors_of(13).sorted().collect_vec();
    let expected = vec![1, 13];
    assert_eq!(result, expected);

    let result = super::divisors_of(15).sorted().collect_vec();
    let expected = vec![1, 3, 5, 15];
    assert_eq!(result, expected);
}
