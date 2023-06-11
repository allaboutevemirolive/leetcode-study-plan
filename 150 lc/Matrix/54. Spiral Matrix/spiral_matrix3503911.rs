// https://leetcode.com/problems/spiral-matrix/solutions/3503911/rust-simulation-100/
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::new();
        let rows = matrix.len();
        let cols = matrix[0].len();
        let total = rows * cols;

        let (mut start_row, mut start_col) = (0, 0);
        let (mut end_row, mut end_col) = (rows-1, cols-1);
        while res.len() < total {
            for i in start_col..=end_col {
                if res.len() >= total {break;}
                res.push(matrix[start_row][i]);
            }
            start_row += 1;

            for i in start_row..=end_row {
                if res.len() >= total {break;}
                res.push(matrix[i][end_col]);
            }
            end_col -= 1;

            for i in (start_col..=end_col).rev() {
                if res.len() >= total {break;}
                res.push(matrix[end_row][i]);
            }
            end_row -= 1;

            for i in (start_row..=end_row).rev() {
                if res.len() >= total {break;}
                res.push(matrix[i][start_col]);
            }
            start_col += 1;
        }

        res
    }
}