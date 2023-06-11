// https://leetcode.com/problems/set-matrix-zeroes/solutions/3509005/rust-simple-approach/
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut rows = vec![];
        let mut cols = vec![];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 && (!rows.contains(&i) || !cols.contains(&j)) {
                    rows.push(i);
                    cols.push(j);
                }
            }
        }
        
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if rows.contains(&i) || cols.contains(&j) {
                    matrix[i][j] = 0;
                }
            } 
        }
    }
}