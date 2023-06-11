// https://leetcode.com/problems/combination-sum/solutions/2385783/rust-0ms-100-faster-recursive-simple-and-easy/
impl Solution {
    pub fn combination_sum(nums: Vec<i32>, t: i32) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut res = Vec::with_capacity(100);
        let mut temp = Vec::with_capacity(100);
        Solution::dfs(&nums, &mut res, &mut temp, t, len, 0);
        res
    }
    fn dfs(nums: &Vec<i32>, res: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, t: i32, len: usize, i: usize) {
        if t < 0 || i >= len {
            return;
        } else if t == 0 {
            res.push(temp.clone());
            return;
        }
        let x = nums[i];
        temp.push(x);
        Solution::dfs(nums, res, temp, t - x, len, i);
        temp.pop();
        Solution::dfs(nums, res, temp, t, len, i + 1);
    }
}