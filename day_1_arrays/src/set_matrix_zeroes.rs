impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }


        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

    }
}