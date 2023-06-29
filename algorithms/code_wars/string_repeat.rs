// Write a function that accepts an integer n and a string s as parameters, and returns a string of s repeated exactly n times.
fn repeat_str(src: &str, count: usize) -> String {
    src.repeat(count)
}

#[test]
fn example_tests() {
    assert_eq!(repeat_str("a", 4), "aaaa");
    assert_eq!(repeat_str("hello ", 3), "hello hello hello ");
    assert_eq!(repeat_str("abc", 2), "abcabc");
}
