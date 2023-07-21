fn greet(name: &str) -> String {
    let capitalized_name = match name.chars().next() {
        Some(first_char) => first_char.to_uppercase().to_string() + &name[1..].to_lowercase(),
        None => String::new(),
    };

    format!("Hello {}!", capitalized_name)
}

#[cfg(test)]
mod tests {
    use super::greet;

    const ERR: &str = "\nYour result (left) did not match the expected output (right).";

    #[test]
    fn returns_expected() {
        assert_eq!(greet("riley"), "Hello Riley!", "{ERR}");
        assert_eq!(greet("JACK"), "Hello Jack!", "{ERR}");
    }
}
