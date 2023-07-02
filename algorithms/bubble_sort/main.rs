pub fn bubble_sort(arr: &mut Vec<i32>) {
    let mut swapped = true;
    while swapped {
        // No swap means array is sorted.
        swapped = false;
        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true
            }
        }
    }
}

fn main() {
    let mut vec_test: Vec<i32> = vec![6, 5, 4, 3, 2, 1];
    println!("Before: {:?}", &vec_test);
    bubble_sort(&mut vec_test);
    println!("After: {:?}", &vec_test);
}