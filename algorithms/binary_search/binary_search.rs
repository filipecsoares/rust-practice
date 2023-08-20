fn binary_search(ls: Vec<u32>, num: u32) -> isize{
    let mut min: usize = 0;
    let mut max: usize = ls.len();
    while max >= min {
        let index = (max + min) / 2;
        println!("i {}, min {}, max {}", index, min, max);
        if ls[index] == num {
            return index as isize;
        } else if num > ls[index] {
            min = index + 1;
        } else {
            max = index - 1;
        }
    }
    -1
}

#[test]
fn test_binary_search() {
    let ls = vec![1, 2, 3, 4, 5];
    let num = 3;
    assert_eq!(binary_search(ls, num), 2);
}