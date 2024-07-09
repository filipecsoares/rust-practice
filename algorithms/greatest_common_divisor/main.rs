// Euclid's algorithm to find the greatest common divisor
// Compute the greatest common divisor ofCompute the greatest common divisor of two nonnegative integers p and q as follows:
// If q is 0, the answer is p. If not, divide p by q and take the remainder r. 
// The answer is the greatest common divisor of q and r.
fn gcd(p: u32, q: u32) -> u32 {
    if q == 0 {
        return p;
    }
    let r = p % q;
    return gcd(q, r);
}

#[test]
fn gcd_test() {
    assert_eq!(0, gcd(0, 0));
    assert_eq!(1, gcd(1, 0));
    assert_eq!(1, gcd(1, 1));
    assert_eq!(2, gcd(4, 2));
    assert_eq!(3, gcd(9, 6));
    assert_eq!(5, gcd(10, 5));
    assert_eq!(8, gcd(136, 16));
    assert_eq!(13, gcd(13, 0));
    assert_eq!(21, gcd(42, 21));
}