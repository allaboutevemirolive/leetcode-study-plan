// https://leetcode.com/problems/rotate-image/solutions/2504176/rust-0-1-ms/
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut matrix_clone = matrix.clone();
        for i in 0..matrix.len() {
            for n in 0..matrix[0].len() {
                matrix[i][matrix_clone[i].len() - n - 1] = matrix_clone[n][i];
            }
           
        }
    }
}