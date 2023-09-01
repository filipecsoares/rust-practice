use std::collections::HashMap;

// Recursive Fibonacci with memoization
fn fib(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n == 0 || n == 1 {
        return n;
    } else if memo.contains_key(&n) {
        return *memo.get(&n).unwrap();
    } else {
        let result = fib(n - 1, memo) + fib(n - 2, memo);
        memo.insert(n, result);
        return result;
    }
}

fn main() {
    let n5 = 5;
    let mut memo: HashMap<u64, u64> = HashMap::new();
    let result = fib(n5, &mut memo);
    println!("Fibonacci({}) = {}", n5, result);
    
    let n10 = 10;
    let result = fib(n10, &mut memo);
    println!("Fibonacci({}) = {}", n10, result);
}