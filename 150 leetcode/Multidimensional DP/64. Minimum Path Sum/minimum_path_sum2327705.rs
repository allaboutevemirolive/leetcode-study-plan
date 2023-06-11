// https://leetcode.com/problems/minimum-path-sum/solutions/2327705/clean-rust-1-dim-dp-solution/
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
        let mut dp = vec![i32::MAX; m + 1];

        dp[1] = 0;
        for i in 0..n {
            for j in 0..m {
                let j_dp = j + 1; // padding correction
                dp[j_dp] = grid[i][j] + dp[j_dp].min(dp[j_dp - 1]);
            }
        }

        dp[m]
    }
}