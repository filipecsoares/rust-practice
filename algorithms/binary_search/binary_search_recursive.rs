fn binary_search(ls: &Vec<u32>, num: u32, min: usize, max: usize) -> isize {
    let index = (max + min) / 2;
    if max < min {
        -1
    }
    if num == ls[index] {
        return index as isize;
    } else if num > ls[index] {
        binary_search(ls, num, index + 1, max)
    } else {
        binary_search(ls, num, min, index - 1)
    }
}

#[test]
fn test_binary_search() {
    let ls = vec![1, 2, 3, 4, 5];
    let num = 3;
    assert_eq!(binary_search(&ls, num, 0, ls.len() - 1), 2);
}