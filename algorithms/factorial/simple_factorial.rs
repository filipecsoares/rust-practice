use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("Hello, world! {}", input);
    let num: u32 = input.trim().parse().expect("Not a number");
    println!("Factorial of {} is {}", num, factorial(num));
}

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

#[test]
fn test_factorial() {
    assert_eq!(factorial(0), 0);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(3), 6);
    assert_eq!(factorial(5), 120);
}
