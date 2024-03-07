#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() { return Err(Error::SpanTooLong); }
    if span == 0 { return Ok(1u64); }
    if string_digits.matches(char::is_alphabetic).collect::<Vec<&str>>().len() > 0 {
        let digit = string_digits.matches(char::is_alphabetic).collect::<Vec<&str>>();
        return Err(Error::InvalidDigit(digit
                .first().unwrap()
                .to_owned()
                .to_string()
                .pop().unwrap()));
    }
    let digits: Vec<u64> = string_digits
        .split("")
        .map(|s| s.parse())
        .filter(|s| match s {
            Ok(_) => true,
            Err(_) => false,
        })
        .map(|s| s.unwrap())
        .collect();
    Ok(window_values(digits, span)
            .first()
            .unwrap()
            .to_owned())
}

fn window_values(digits: Vec<u64>, span: usize) -> Vec<u64> {
    let mut window_values = digits
        .windows(span)
        .map(|w| w.to_vec())
        .map(|v| v.iter().fold(1,|acc, i| acc * i))
        .collect::<Vec<u64>>();
    window_values.sort();
    window_values.reverse();
    window_values
}

#[test]
fn return_is_a_result() {
    assert!(lsp("29", 2).is_ok());
}

#[test]
fn find_the_largest_product_when_span_equals_length() {
    assert_eq!(Ok(18), lsp("29", 2));
}

#[test]
fn find_the_largest_product_of_two_with_numbers_in_order() {
    assert_eq!(Ok(72), lsp("0123456789", 2));
}

#[test]
fn find_the_largest_product_of_two_with_numbers_not_in_order() {
    assert_eq!(Ok(48), lsp("576802143", 2));
}

#[test]
fn find_the_largest_product_of_three_with_numbers_in_order() {
    assert_eq!(Ok(504), lsp("0123456789", 3));
}

#[test]
fn find_the_largest_product_of_three_with_numbers_not_in_order() {
    assert_eq!(Ok(270), lsp("1027839564", 3));
}

#[test]
fn find_the_largest_product_of_five_with_numbers_in_order() {
    assert_eq!(Ok(15120), lsp("0123456789", 5));
}

#[test]
fn span_of_six_in_a_large_number() {
    assert_eq!(
        Ok(23520),
        lsp("73167176531330624919225119674426574742355349194934", 6)
    );
}

#[test]
fn returns_zero_if_number_is_zeros() {
    assert_eq!(Ok(0), lsp("0000", 2));
}

#[test]
fn returns_zero_if_all_products_are_zero() {
    assert_eq!(Ok(0), lsp("99099", 3));
}

#[test]
fn a_span_is_longer_than_number_is_an_error() {
    assert_eq!(Err(Error::SpanTooLong), lsp("123", 4));
}

#[test]
fn an_empty_string_and_no_span_returns_one() {
    assert_eq!(Ok(1), lsp("", 0));
}

#[test]
fn a_non_empty_string_and_no_span_returns_one() {
    assert_eq!(Ok(1), lsp("123", 0));
}

#[test]
fn empty_string_and_non_zero_span_is_an_error() {
    assert_eq!(Err(Error::SpanTooLong), lsp("", 1));
}

#[test]
fn a_string_with_non_digits_is_an_error() {
    assert_eq!(Err(Error::InvalidDigit('a')), lsp("1234a5", 2));
}
