// https://leetcode.com/problems/remove-element/solutions/2823845/rust-0ms/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut ins_idx = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[ins_idx] = nums[i];
                ins_idx += 1;
            }
        }
        ins_idx as i32
    }
}