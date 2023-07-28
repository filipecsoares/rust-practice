impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut count: i32 = 0;
        let mut n = num;
        while n > 0 {
            if n % 2 == 0 {
                n /= 2;
            } else {
                n -= 1;
            }
            count += 1;
        }
        count
    }
}
