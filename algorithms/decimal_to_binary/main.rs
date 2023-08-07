fn to_binary(decimal: u32) -> String {
    let mut binary = String::new();
    let mut n = decimal;
    while n > 0 {
        let remainder = n % 2;
        binary.push_str(&remainder.to_string());
        n /= 2;
    }
    binary.chars().rev().collect()
}

// easy way
fn to_binary_easy(n: u32) -> String {
    format!("{:b}", n)
}

#[test]
fn test_to_binary() {
    assert_eq!(to_binary(1), "1");
    assert_eq!(to_binary(2), "10");
    assert_eq!(to_binary(3), "11");
    assert_eq!(to_binary(4), "100");
    assert_eq!(to_binary(5), "101");
    assert_eq!(to_binary(6), "110");
    assert_eq!(to_binary(7), "111");
    assert_eq!(to_binary(8), "1000");
    assert_eq!(to_binary(9), "1001");
    assert_eq!(to_binary(10), "1010");
    assert_eq!(to_binary(11), "1011");
    assert_eq!(to_binary(12), "1100");
    assert_eq!(to_binary(13), "1101");
    assert_eq!(to_binary(14), "1110");
    assert_eq!(to_binary(15), "1111");
    assert_eq!(to_binary(16), "10000");
    assert_eq!(to_binary(17), "10001");
    assert_eq!(to_binary(18), "10010");
    assert_eq!(to_binary(19), "10011");
    assert_eq!(to_binary(20), "10100");
    assert_eq!(to_binary(21), "10101");
    assert_eq!(to_binary(22), "10110");
    assert_eq!(to_binary(23), "10111");
    assert_eq!(to_binary(24), "11000");
    assert_eq!(to_binary(25), "11001");
    assert_eq!(to_binary(26), "11010");
    assert_eq!(to_binary(27), "11011");
    assert_eq!(to_binary(28), "11100");
    assert_eq!(to_binary(29), "11101");
    assert_eq!(to_binary(30), "11110");
    assert_eq!(to_binary(31), "11111");
    assert_eq!(to_binary(32), "100000");
    assert_eq!(to_binary(33), "100001");
    assert_eq!(to_binary(34), "100010");
    assert_eq!(to_binary(35), "100011");
    assert_eq!(to_binary(36), "100100");
    assert_eq!(to_binary(37), "100101");
    assert_eq!(to_binary(38), "100110");
}