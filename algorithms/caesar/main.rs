fn caesar(cipher: &str, shift: u8) -> String {
    cipher.chars().map(|c| {
        if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                // modulo the distance to keep character range
                (first + (c as u8 + shift - first) % 26) as char
            } else {
                c
            }
    }).collect()
}

#[test]
fn test_caesar_shift_13() {
    assert_eq!(caesar("rust", 13), "ehfg");
}

#[test]
fn test_caesar_shift_2() {
    assert_eq!(caesar("abc", 2), "cde");
}

#[test]
fn test_caesar_shift_blank() {
    assert_eq!(caesar("", 13), "");
}