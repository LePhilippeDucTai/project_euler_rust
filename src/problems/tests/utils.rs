use itertools::Itertools;

use super::fibonacci;
use super::is_even;
use super::merge_sorted_2;
use super::mergesort;
use super::pgcd;

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

#[test]
fn test_fibonacci() {
    let fib = fibonacci();
    let even = fib.take(10).collect_vec();
    assert_eq!(even, vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55])
}

#[test]
fn test_is_even() {
    let a: i32 = 100;
    let b: i64 = 2103;
    let c: i128 = 58104592;

    assert_eq!(is_even(&a), true);
    assert_eq!(is_even(&b), false);
    assert_eq!(is_even(&c), true);
}

#[test]
fn test_gcd() {
    let result = pgcd(782, 221);
    let expected = 17;
    assert_eq!(result, expected);

    let result = pgcd(221, 782);
    let expected = 17;
    assert_eq!(result, expected);

    let result = pgcd(1332, 228);
    let expected = 12;
    assert_eq!(result, expected);
}

#[test]
fn test_mergesort() {
    let mut v: [i32; 6] = [4, 2, 9, 10, 2, 1];
    let result = mergesort(&mut v);
    assert_eq!(result, vec![1, 2, 2, 4, 9, 10]);
}

#[test]
fn test_merge_sorted_2() {
    let v = vec![0, 4, 5, 5, 6, 9];
    let w = vec![1, 2, 6, 9, 10];
    let result = merge_sorted_2(v, w);
    assert_eq!(result, vec![0, 1, 2, 4, 5, 5, 6, 6, 9, 9, 10])
}
