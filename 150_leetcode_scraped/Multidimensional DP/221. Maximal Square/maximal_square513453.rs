// https://leetcode.com/problems/maximal-square/solutions/513453/rust-4ms-dp-solution/
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let (row, col) = (matrix.len(), matrix[0].len());
        let mut dp: Vec<Vec<usize>> = vec![vec![0; col]; row];
        let mut max = 0;
        for i in 0..row {
            for j in 0..col {
                if matrix[i][j] == '1' {
                    let mut min = if i * j > 0 { dp[i - 1][j - 1] } else { 0 };
                    if i > 0 {
                        min = std::cmp::min(min, dp[i - 1][j]);
                    }
                    if j > 0 {
                        min = std::cmp::min(min, dp[i][j - 1]);
                    }
                    dp[i][j] = min + 1;
                    max = std::cmp::max(max, dp[i][j]);
                }
            }
        }
        (max * max) as i32
    }
}