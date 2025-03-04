// Euclid's algorithm to find the greatest common divisor
fn find_gcd_recursive(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    let r = a % b;
    return find_gcd_recursive(b, r);
}

fn find_gcd_iterative(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    return a;
}

#[test]
fn test_find_greatest_common_divisor_recursive() {
    assert_eq!(find_gcd_recursive(2, 2), 2);
    assert_eq!(find_gcd_recursive(10, 5), 5);
    assert_eq!(find_gcd_recursive(15, 20), 5);
    assert_eq!(find_gcd_recursive(5, 3), 1);
}

#[test]
fn test_find_greatest_common_divisor_iterative() {
    assert_eq!(find_gcd_iterative(2, 2), 2);
    assert_eq!(find_gcd_iterative(10, 5), 5);
    assert_eq!(find_gcd_iterative(15, 20), 5);
    assert_eq!(find_gcd_iterative(5, 3), 1);
}