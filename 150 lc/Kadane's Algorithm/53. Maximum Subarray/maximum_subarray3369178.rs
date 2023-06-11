// https://leetcode.com/problems/maximum-subarray/solutions/3369178/rust-clean-and-simple/
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;
        let mut sum = 0;
        for n in nums {
            sum += n;
            max = max.max(sum);
            sum = sum.max(0);
        }
        max
    }
}