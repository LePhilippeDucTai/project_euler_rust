use super::{is_triangle, words_to_values};

#[test]
fn test_solve() {
    let words = vec!["SKY".to_string()];
    let actual = words_to_values(words);
    let expected = [55];
    assert_eq!(actual, expected)
}
#[test]
fn test_triangle() {
    let value = 55;
    let actual = is_triangle(value);
    let expected = true;
    assert_eq!(actual, expected);

    let value = 67;
    let actual = is_triangle(value);
    let expected = false;
    assert_eq!(actual, expected);
}
