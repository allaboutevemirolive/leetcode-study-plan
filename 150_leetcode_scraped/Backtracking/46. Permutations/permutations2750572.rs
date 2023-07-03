// https://leetcode.com/problems/permutations/solutions/2750572/rust-backtrack-swap-in-nums/
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result :  Vec<Vec<i32>> = vec![];
        let mut nums = nums;
        Solution::backtrack(0usize, &mut nums, &mut result);
        result
    }
    
    pub fn backtrack(first : usize, nums: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if first == nums.len() {
            result.push(nums.clone());
            return;
        }
        for i in first..nums.len() {
            nums.swap(first, i);
            Solution::backtrack(first+1, nums, result);
            nums.swap(first, i);
        }
    }
}