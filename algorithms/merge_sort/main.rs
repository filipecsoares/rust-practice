fn merge_sort(arr: Vec<i32>) -> Vec<i32> {
    // Base case: if the array has 0 or 1 elements, it is already sorted
    if arr.len() <= 1 {
        return arr;
    }
    let middle_index = arr.len() / 2;
    let left_half = &arr[..middle_index];
    let right_half = &arr[middle_index..];

    // Recursively sort each half
    let sorted_left_half = merge_sort(left_half.to_vec());
    let sorted_right_half = merge_sort(right_half.to_vec());

    // Merge the sorted halves
    return merge(&sorted_left_half, &sorted_right_half);
}

fn merge(left_half: &Vec<i32>, right_half: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut left_index = 0;
    let mut right_index = 0;

    while left_index < left_half.len() && right_index < right_half.len() {
        if left_half[left_index] < right_half[right_index] {
            result.push(left_half[left_index]);
            left_index += 1;
        } else {
            result.push(right_half[right_index]);
            right_index += 1;
        }
    }
    // Append remaining elements
    result.extend_from_slice(&left_half[left_index..]);
    result.extend_from_slice(&right_half[right_index..]);

    result
}

#[test]
fn empty_array_test() {
  let arr: [i32; 0] = [];
  let sorted_arr = merge_sort(arr.to_vec());
  assert_eq!(sorted_arr, vec![]);
}

#[test]
fn sorted_array_test() {
  let arr = [1, 2, 3, 4];
  let sorted_arr = merge_sort(arr.to_vec());
  assert_eq!(sorted_arr, vec![1, 2, 3, 4]);
}

#[test]
fn reverse_sorted_array_test() {
  let arr = [4, 3, 2, 1];
  let sorted_arr = merge_sort(arr.to_vec());
  assert_eq!(sorted_arr, vec![1, 2, 3, 4]);
}

#[test]
fn unsorted_array_test() {
  let arr = [6, 5, 3, 1, 8, 7, 2, 4];
  let sorted_arr = merge_sort(arr.to_vec());
  assert_eq!(sorted_arr, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn array_with_duplicates_test() {
  let arr = [3, 1, 1, 4, 2, 2];
  let sorted_arr = merge_sort(arr.to_vec());
  assert_eq!(sorted_arr, vec![1, 1, 2, 2, 3, 4]);
}