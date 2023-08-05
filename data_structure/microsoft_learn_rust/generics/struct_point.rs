#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer_and_boolean = Point { x: 5, y: false };
    let float_and_string = Point { x: 1.0, y: "hey" };
    let integer_and_float = Point { x: 5, y: 4.0 };
    let both_integer = Point { x: 10, y: 30 };
    let both_boolean = Point { x: true, y: true };
    println!("{:?}", integer_and_boolean);
    println!("{:?}", float_and_string);
    println!("{:?}", integer_and_float);
    println!("{:?}", both_integer);
    println!("{:?}", both_boolean);
}