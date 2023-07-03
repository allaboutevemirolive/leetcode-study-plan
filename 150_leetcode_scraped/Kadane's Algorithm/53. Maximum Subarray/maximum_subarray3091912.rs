// https://leetcode.com/problems/maximum-subarray/solutions/3091912/easy-to-understand-rust/
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut cur = 0;

        for num in nums {
            cur = i32::max(num, cur + num);
            ans = i32::max(ans, cur);
        }

        ans
    }
}