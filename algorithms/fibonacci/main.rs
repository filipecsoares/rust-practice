fn nth_fibonacci(num: u32) -> u32 {
    match num {
        0 | 1 => num,
        _ => nth_fibonacci(num - 1) + nth_fibonacci(num - 2),
    }
}

#[test]
fn nth_fibonacci_test() {
    assert_eq!(0, nth_fibonacci(0));
    assert_eq!(1, nth_fibonacci(1));
    assert_eq!(1, nth_fibonacci(2));
    assert_eq!(2, nth_fibonacci(3));
    assert_eq!(3, nth_fibonacci(4));
    assert_eq!(5, nth_fibonacci(5));
    assert_eq!(8, nth_fibonacci(6));
    assert_eq!(13, nth_fibonacci(7));
    assert_eq!(21, nth_fibonacci(8));
}
