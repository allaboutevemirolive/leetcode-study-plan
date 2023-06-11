// https://leetcode.com/problems/remove-element/solutions/3497436/rust-o-n-solution/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }
}