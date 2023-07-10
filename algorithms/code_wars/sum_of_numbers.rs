fn get_sum(a: i64, b: i64) -> i64 {
    // let mut smallest: i64 = a;
    // let mut biggest: i64 = b;
    // if a > b {
    //     smallest = b;
    //     biggest = a;
    // }
    // let mut total: i64 = 0;
    // for i in smallest..=biggest {
    //     total += i;
    // }
    // total
    (a.min(b)..=a.max(b)).sum()
}

#[cfg(test)]
mod tests {
    use super::get_sum;

    #[test]
    fn sample_tests() {
        assert_eq!(get_sum(0, 1), 1);
        assert_eq!(get_sum(1, 2), 3);
        assert_eq!(get_sum(5, -1), 14);
        assert_eq!(get_sum(505, 4), 127759);
    }
}
