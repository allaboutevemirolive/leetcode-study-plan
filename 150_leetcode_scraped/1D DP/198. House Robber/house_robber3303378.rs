// https://leetcode.com/problems/house-robber/solutions/3303378/rust-easy-solution-with-memo/
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut res = vec![];
        let mut memo = vec![-1; nums.len()];
        Self::dfs(&nums, 0, 0, &mut res, &mut memo);
        Self::dfs(&nums, 1, 0, &mut res, &mut memo);
        *res.iter().max().unwrap()
    }
    pub fn dfs(hous: &Vec<i32>, start: usize, mut robbed: i32, res: &mut Vec<i32>, memo:&mut Vec<i32>) {
        if start >= hous.len() {
            return res.push(robbed);
        }
        robbed += hous[start];
        if robbed > memo[start] {
            memo[start] = robbed
        } else { 
            return
        }
        Self::dfs(hous, start + 2, robbed, res, memo);
        Self::dfs(hous, start + 3, robbed, res, memo);
    }
}