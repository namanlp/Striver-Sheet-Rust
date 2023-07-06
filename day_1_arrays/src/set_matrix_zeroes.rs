// Question Link :  https://leetcode.com/problems/set-matrix-zeroes/

// My solution time complexity : O(m*n)
// My solution space complexity : O(m+n)

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {

        let mut rows = vec![1;matrix.len()];
        let mut columns = vec![1;matrix[0].len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 0 {
                    rows[i] = 0;
                    columns[j] = 0;
                }
            }
        }


        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if rows[i] == 0 || columns[j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

    }
}