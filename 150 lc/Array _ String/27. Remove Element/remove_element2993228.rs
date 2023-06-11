// https://leetcode.com/problems/remove-element/solutions/2993228/rust-simple-solution/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        
       nums.retain(|&x| val != x);
        nums.len() as i32
    }
}