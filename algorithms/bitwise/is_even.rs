// Check if a number is even without using modulo.
fn is_even(num: i32) -> bool {
    num & 1 == 0
}

#[test]
fn test_even() {
    assert_eq!(is_even(1), false);
    assert_eq!(is_even(2), true);
    assert_eq!(is_even(3), false);
    assert_eq!(is_even(4), true);
    assert_eq!(is_even(-1), false);
    assert_eq!(is_even(5), false);
    assert_eq!(is_even(100), true);
}