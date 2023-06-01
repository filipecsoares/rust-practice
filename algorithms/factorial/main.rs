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

fn factorial_recursive(num: u32) -> u32 {
    if num == 0 || num == 1 {
        return num;
    }
    return num * factorial_recursive(num - 1);
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

    #[test]
    fn test_factorial_recursive() {
        assert_eq!(factorial_recursive(0), 0);
        assert_eq!(factorial_recursive(1), 1);
        assert_eq!(factorial_recursive(2), 2);
        assert_eq!(factorial_recursive(3), 6);
        assert_eq!(factorial_recursive(4), 24);
        assert_eq!(factorial_recursive(5), 120);
    }
}
