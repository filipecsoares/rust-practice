/// Sorts a mutable slice using in-place insertion sort algorithm.
///
/// Time complexity is `O(n^2)`, where `n` is the number of elements.
/// Space complexity is `O(1)` as it sorts elements in-place.
pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        let cur = arr[i];
        while j > 0 && cur < arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = cur;
    }
}

pub fn sort_swap(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut arr: [u8; 0] = [];
        let expected: [u8; 0] = [];
        insertion_sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn one_element() {
        let mut arr: [char; 1] = ['a'];
        let expected: [char; 1] = ['a'];
        insertion_sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn already_sorted() {
        let mut arr: [&str; 3] = ["a", "b", "c"];
        let expected: [&str; 3] = ["a", "b", "c"];
        insertion_sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn basic() {
        let mut arr: [&str; 4] = ["d", "a", "c", "b"];
        let expected: [&str; 4] = ["a", "b", "c", "d"];
        insertion_sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
        let expected: Vec<&str> = vec!["a", "b", "c", "d", "e"];
        insertion_sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn repeated_elements() {
        let mut arr: Vec<usize> = vec![542, 542, 542, 542];
        let expected: Vec<usize> = vec![542, 542, 542, 542];
        insertion_sort(&mut arr);
        assert_eq!(arr, expected);
    }
}
