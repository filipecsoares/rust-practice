// Euclid's algorithm to find the greatest common divisor
fn find_gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    let r = a % b;
    return find_gcd(b, r);
}

#[test]
fn test_find_greatest_common_divisor() {
    assert_eq(find_gcd(2, 2), 2)
    assert_eq(find_gcd(10, 5), 5)
    assert_eq(find_gcd(15, 20), 5)
    assert_eq(find_gcd(5, 3), 1)
}