fn factorial(num: u32) -> u32 {
    if num == 0 {
        return 0;
    }
    let mut total: u32 = 1;
    for i in 2..=num {
        total *= i;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 0);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
    }
}
