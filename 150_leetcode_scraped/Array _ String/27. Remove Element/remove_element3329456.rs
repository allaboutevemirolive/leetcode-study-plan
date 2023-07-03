// https://leetcode.com/problems/remove-element/solutions/3329456/vector-method-solution-for-rust-simple/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }
}