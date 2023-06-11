// https://leetcode.com/problems/remove-element/solutions/250157/rust-solution/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut total = nums.len();
        for i in (0..total).rev() {
            if !nums.is_empty() && nums[i] == val {
                nums.remove(i);
                total -= 1;
            }
        }
        total as i32
    }
}