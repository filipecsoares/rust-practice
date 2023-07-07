fn is_square(n: i64) -> bool {
    for i in 0..=n {
        if i * i == n {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::is_square;

    #[test]
    fn fixed_tests() {
        assert_eq!(is_square(-1), false, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(0), true, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(3), false, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(4), true, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(25), true, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(26), false, "\nYour answer (left) is not the expected answer (right).");
    }
}