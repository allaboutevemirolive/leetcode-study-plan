// https://leetcode.com/problems/combination-sum/solutions/2376653/rust-dp-similar-to-coin-change-ii/
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let m = target as usize + 1;
        let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![]; m];
        dp[0].push(vec![]);

        for candidate in candidates {
            let candidate = candidate as usize;
            for i in candidate..m {
                for combination in dp[i - candidate].clone() {
                    let mut new_comb = combination;
                    new_comb.push(candidate as i32);
                    dp[i].push(new_comb);
                }
            }
        }
        dp.remove(m - 1)
    }