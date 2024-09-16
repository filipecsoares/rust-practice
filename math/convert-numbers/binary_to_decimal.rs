fn convert_binary_to_decimal(binary: &str) -> u32 {
    //i32::from_str_radix(binary, 2).unwrap()
    let mut result = 0;
    for (i, c) in binary.chars().rev().enumerate() {
        let digit = c.to_digit(10).unwrap();
        result += digit * 2_u32.pow(i as u32);
    }
    result
}


#[test]
fn test_convert_binary_to_decimal() {
    assert_eq!(convert_binary_to_decimal("0"), 0);
    assert_eq!(convert_binary_to_decimal("1"), 1);
    assert_eq!(convert_binary_to_decimal("10"), 2);
    assert_eq!(convert_binary_to_decimal("11"), 3);
    assert_eq!(convert_binary_to_decimal("100"), 4);
    assert_eq!(convert_binary_to_decimal("101"), 5);
    assert_eq!(convert_binary_to_decimal("110"), 6);
    assert_eq!(convert_binary_to_decimal("111"), 7);
    assert_eq!(convert_binary_to_decimal("1000"), 8);
    assert_eq!(convert_binary_to_decimal("1001"), 9);
    assert_eq!(convert_binary_to_decimal("1010"), 10);
    assert_eq!(convert_binary_to_decimal("1011"), 11);
    assert_eq!(convert_binary_to_decimal("1100"), 12);
    assert_eq!(convert_binary_to_decimal("1101"), 13);
    assert_eq!(convert_binary_to_decimal("1110"), 14);
    assert_eq!(convert_binary_to_decimal("1111"), 15);
}
