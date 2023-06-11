// https://leetcode.com/problems/remove-duplicates-from-sorted-array/solutions/2970179/rust-two-pointers-functional-style/
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        (0..nums.len()).fold(0, |k, idx| match k as usize {
            prev if nums[prev]==nums[idx] => k,
            prev => { nums[prev+1]=nums[idx]; k+1 }
        }) + 1
    }
}