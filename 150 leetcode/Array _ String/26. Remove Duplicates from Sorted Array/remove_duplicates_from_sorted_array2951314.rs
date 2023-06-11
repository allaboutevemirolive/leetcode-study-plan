// https://leetcode.com/problems/remove-duplicates-from-sorted-array/solutions/2951314/rust-solution-in-2-lines/
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}