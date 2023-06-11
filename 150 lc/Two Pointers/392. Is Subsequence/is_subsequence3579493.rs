// https://leetcode.com/problems/is-subsequence/solutions/3579493/rust-overkill-dp-approach-0ms/
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0 { return true; }

        let [sb, tb] = [s.as_bytes(), t.as_bytes()];
        let mut dp = vec![vec![false; t.len()]; s.len()];
        for j in 0..t.len() {
            dp[0][j] = sb[0] == tb[j];
        }
        for i in 1..s.len() {
            let mut prev_row_match = false;
            for j in 1..t.len() {
                prev_row_match |= dp[i-1][j-1];
                dp[i][j] = prev_row_match && sb[i] == tb[j];
            }
            if !prev_row_match { return false; }
        }

        dp.last().map_or(true, |last_row| last_row.iter().any(|&b| b))
    }
}