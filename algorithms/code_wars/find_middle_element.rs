fn gimme(input_array: [i32; 3]) -> usize {
    let mut sorted_array = input_array; // Create a mutable copy of the input array
    sorted_array.sort(); // Sort the array in ascending order

    // Find the index of the middle element in the sorted array
    let middle_index = input_array
        .iter()
        .position(|&x| x == sorted_array[1])
        .unwrap();
    middle_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gimme() {
        assert_eq!(gimme([2, 3, 1]), 0);
        assert_eq!(gimme([-2, -3, -1]), 0);
        assert_eq!(gimme([5, 10, 14]), 1);
    }
}
