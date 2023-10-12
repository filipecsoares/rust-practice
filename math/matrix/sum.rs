pub fn sum_matrix_iter(matrix1: Vec<Vec<i32>>, matrix2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    assert_eq!(matrix1.len(), matrix2.len());
    assert_eq!(matrix1[0].len(), matrix2[0].len());
    matrix1.iter()
        .zip(matrix2.iter())
        .map(|(row1, row2)| {
            row1.iter()
                .zip(row2.iter())
                .map(|(a, b)| {
            *a + *b
        }).collect()
    }).collect()
}

pub fn sum_matrix_for(matrix1: Vec<Vec<i32>>, matrix2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    assert_eq!(matrix1.len(), matrix2.len());
    assert_eq!(matrix1[0].len(), matrix2[0].len());
    let mut result: Vec<Vec<i32>> = vec![vec![0; matrix1[0].len()]; matrix1.len()];
    for i in 0..matrix1.len() {
        for j in 0..matrix1[0].len() {
            result[i][j] = matrix1[i][j] + matrix2[i][j];
        }
    }
    result
}

#[test]
fn test_sum_matrix_for() {
    assert_eq!(sum_matrix_for(vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]), vec![vec![6, 8], vec![10, 12]]);
}

#[test]
fn test_sum_matrix_iter() {
    assert_eq!(sum_matrix_iter(vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]), vec![vec![6, 8], vec![10, 12]]);
}