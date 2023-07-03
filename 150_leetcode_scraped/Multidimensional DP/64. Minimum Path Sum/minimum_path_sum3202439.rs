// https://leetcode.com/problems/minimum-path-sum/solutions/3202439/rust-bottom-up-dp-solution/
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp = vec![vec![0; m]; n];
        dp[0][0] = grid[0][0];
        for i in 1..m {
            dp[0][i] += dp[0][i - 1] + grid[0][i]
        }
        for i in 1..n {
            dp[i][0] += dp[i - 1][0] + grid[i][0]
        }
        for i in 1..n {
            for j in 1..m {
                dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]) + grid[i][j];
            }
        }
        dp[n - 1][m - 1]        
    }
}