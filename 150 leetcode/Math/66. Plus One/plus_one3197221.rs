// https://leetcode.com/problems/plus-one/solutions/3197221/rust-100-time-complexity-recursive-solution/
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        return Solution::helper(&mut digits);
    }
    pub fn helper(nums: &mut Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if nums[n-1] < 9 {
            nums[n-1] += 1;
            return nums.to_vec();
        }
        if nums.len() == 1 {
            return vec![1,0];
        }
        nums.pop();
        let mut new_nums = Solution::helper(nums);
        new_nums.push(0);
        return new_nums.to_vec();
    }
}