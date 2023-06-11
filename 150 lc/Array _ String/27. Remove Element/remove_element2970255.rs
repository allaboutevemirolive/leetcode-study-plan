// https://leetcode.com/problems/remove-element/solutions/2970255/rust-functional-style-one-liner/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        (0..nums.len()).fold((0, 0), |(k, d), idx| if nums[idx]==val { (k, d+1) } else { nums[idx-d]=nums[idx]; (k+1, d) }).0
    }
}