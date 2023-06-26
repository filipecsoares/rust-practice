fn number(lines: &[&str]) -> Vec<String> {
    let mut count = 0;
    lines.iter().map(|s| { count += 1; format!("{}: {}", count, s.to_string())}).collect()
}

#[cfg(test)]
mod tests {
    use super::number;

    fn dotest(arr: &[&str], expected: &[&str]) {
        let actual = number(arr);
        assert!(actual == expected, 
            "With lines= {arr:?}\nExpected {expected:?}\nBut got {actual:?}")
    }
    
    #[test]
    fn sample_tests() {
        dotest(&[], &[]);
        dotest(&["a", "b", "c"], &["1: a", "2: b", "3: c"]);
        dotest(&["", "", ""], &["1: ", "2: ", "3: "]);
        dotest(&["", "b", "", ""], &["1: ", "2: b", "3: ", "4: "]);
    }
}