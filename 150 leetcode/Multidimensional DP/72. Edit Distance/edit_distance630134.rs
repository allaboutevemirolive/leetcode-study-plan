// https://leetcode.com/problems/edit-distance/solutions/630134/rust-dp-solution-2mb-0ms-both-100/
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (s1, s2) = (word1.as_bytes(), word2.as_bytes());
        let mut dp = Vec::new();
        for j in 0..=s2.len() as i32 {
            dp.push(j)
        }

        let mut last_dp = dp.clone();

        for i in 1..=s1.len() {
            for j in 0..=s2.len() {
                if j == 0 {
                    dp[j] += 1;
                } else if s1[i - 1] == s2[j - 1] {
                    dp[j] = last_dp[j - 1];
                } else {
                    dp[j] = dp[j].min(last_dp[j - 1]).min(dp[j - 1]);
                    dp[j] += 1;
                }
            }
            last_dp.copy_from_slice(&dp);
        }
        dp[s2.len()]
    }
}