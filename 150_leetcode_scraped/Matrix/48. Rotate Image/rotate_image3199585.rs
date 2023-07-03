// https://leetcode.com/problems/rotate-image/solutions/3199585/rust-solution-inverse-and-transpose/
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let rowSize = matrix.len();
        // inverse
        for i in 0..rowSize/2 {
            for j in 0..rowSize {
                matrix[i][j] ^= matrix[rowSize-i-1][j];
                matrix[rowSize-i-1][j] ^= matrix[i][j];
                matrix[i][j] ^= matrix[rowSize-i-1][j];
            }
        }
        // tranpose
        for i in 0..rowSize {
            for j in 0..i {
                matrix[i][j] ^= matrix[j][i];
                matrix[j][i] ^= matrix[i][j];
                matrix[i][j] ^= matrix[j][i];
            }
        }
    }
}