// https://leetcode.com/problems/remove-element/solutions/3232517/rust-fast-simple-solution-0ms/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|num| *num != val);
        nums.len() as i32
    }
}