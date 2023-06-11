// https://leetcode.com/problems/maximum-subarray/solutions/3006100/rust-three-fast-solutions/
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.iter().fold((0 ,nums[0]), |(curSum,maxSum),n|   
            (0.max(curSum + n) , maxSum.max(curSum + n))
        ).1
    }

}