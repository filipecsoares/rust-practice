pub fn multiply_matrix_for(matrix1: Vec<Vec<i32>>, matrix2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    assert_eq!(matrix1[0].len(), matrix2.len());
    let mut result: Vec<Vec<i32>> = vec![vec![0; matrix2[0].len()]; matrix1.len()];
    for i in 0..matrix1.len() {
        for j in 0..matrix2[0].len() {
            for k in 0..matrix1[0].len() {
                result[i][j] += matrix1[i][k] * matrix2[k][j];
            }
        }
    }
    result
}

#[test]
fn test_multiply_matrix_for() {
    assert_eq!(multiply_matrix_for(vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]), vec![vec![19, 22], vec![43, 50]]);
}