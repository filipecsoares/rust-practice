fn sum_sequence_until(num: u32) -> u32 {
    ((num + 1) as f32 * (num as f32 /2.0)) as u32
}

#[test]
fn test_sum_sequence_until() {
    assert_eq!(sum_sequence_until(5), 15);
    assert_eq!(sum_sequence_until(6), 21);
}