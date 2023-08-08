fn to_hex(decimal: u32) -> String {
    let mut hex = String::new();
    let mut n = decimal;
    while n > 0 {
        let remainder = n % 16;
        let mut remainder_str = match remainder {
            10 => "A".to_string(),
            11 => "B".to_string(),
            12 => "C".to_string(),
            13 => "D".to_string(),
            14 => "E".to_string(),
            15 => "F".to_string(),
            _ => remainder.to_string(),
        };
        hex.push_str(&remainder_str);
        n /= 16;
    }
    hex.chars().rev().collect()
}

fn to_hex_easy(n: u32) -> String {
    format!("{:x}", n)
}

#[test]
fn test_to_hex() {
    assert_eq!(to_hex(1), "1");
    assert_eq!(to_hex(2), "2");
    assert_eq!(to_hex(3), "3");
    assert_eq!(to_hex(4), "4");
    assert_eq!(to_hex(5), "5");
    assert_eq!(to_hex(6), "6");
    assert_eq!(to_hex(7), "7");
    assert_eq!(to_hex(8), "8");
    assert_eq!(to_hex(9), "9");
    assert_eq!(to_hex(10), "A");
    assert_eq!(to_hex(11), "B");
    assert_eq!(to_hex(12), "C");
    assert_eq!(to_hex(13), "D");
    assert_eq!(to_hex(14), "E");
    assert_eq!(to_hex(15), "F");
    assert_eq!(to_hex(16), "10");
    assert_eq!(to_hex(17), "11");
    assert_eq!(to_hex(18), "12");
}