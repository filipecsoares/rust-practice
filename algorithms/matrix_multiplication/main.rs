fn main() {
    // matrix A 2x3
    let matrix_a = [[1, 2, 3], [4, 5, 6]];
    // matrix B 3x4
    let matrix_b = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];

    if matrix_a[0].len() != matrix_b.len() {
        panic!("Matrix dimensions do not match to perform matrix multiplication");
    }
    // result matrix C 2x4
    let mut matrix_c = [[0, 0, 0, 0], [0, 0, 0, 0]];
    // all lines of matrix a
    for i in 0..matrix_a.len() {
        // all columns of matrix b
        for j in 0..matrix_b[0].len() {
            // all columns of matrix a
            for k in 0..matrix_a[0].len() {
                // sum of the multiplication of all columns of matrix a and all lines of matrix b
                matrix_c[i][j] += matrix_a[i][k] * matrix_b[k][j];
            }
        }
    }
    println!("{:?}", matrix_c);
}