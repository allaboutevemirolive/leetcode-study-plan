// https://leetcode.com/problems/spiral-matrix/solutions/3505235/rust-solution-1ms-2mb/
impl Solution {
        pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let n_items = matrix.len() * matrix[0].len();

        let mut row_start = 1;
        let mut row_end = matrix.len() - 1;
        let mut col_start = 0;
        let mut col_end = matrix[0].len() - 1;
        let mut i = 0;
        let mut j = 0;

        while result.len() < n_items {
            while j <= col_end && result.len() < n_items {
                result.push(matrix[i][j]);
                if j == col_end {
                    i += 1;
                    break;
                } else {
                    j += 1;
                }
            }
            col_end -= 1;

            while i <= row_end && result.len() < n_items {
                result.push(matrix[i][j]);
                if i == row_end {
                    j -= 1;
                    break;
                } else {
                    i += 1;
                }
            }
            row_end -= 1;

            while j >= col_start && result.len() < n_items {
                result.push(matrix[i][j]);
                if j == col_start {
                    i -= 1;
                    break;
                } else {
                    j -= 1;
                }
            }
            col_start += 1;

            while i >= row_start && result.len() < n_items {
                result.push(matrix[i][j]);
                if i == row_start {
                    j += 1;
                    break;
                } else {
                    i -= 1;
                }
            }
            row_start += 1;
        }

        result
    }

}