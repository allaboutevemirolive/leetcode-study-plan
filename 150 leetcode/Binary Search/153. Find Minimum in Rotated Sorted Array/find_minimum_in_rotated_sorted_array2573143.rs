// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/solutions/2573143/rust-recursive-solution-magical-0ms-runtime-for-all-your-minimum-number-needs/
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0]
        }
        let middle =nums.len() / 2;
        
        if nums[nums.len() - 1] < nums[middle -1 ] {
            return Solution::find_min(nums[middle..nums.len()].to_vec())
        } 
        return Solution::find_min(nums[0..middle].to_vec())
    }
}
