// https://leetcode.com/problems/rotate-image/solutions/3395239/rust-o-n-based-on-square-symmetry-transform/
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n {
            for j in i..n {
                if i != j {
                    let (a, b) = (matrix[i][j], matrix[j][i]);
                    matrix[j][i] = a; 
                    matrix[i][j] = b;
                }
            }
        }
        let mp = (n + 1) / 2;
        for i in 0..n {
            for j in 0..mp {
                let (a, b) = (matrix[i][j], matrix[i][n-j-1]);
                matrix[i][n-j-1] = a; 
                matrix[i][j] = b;
            }
        }
    }
}