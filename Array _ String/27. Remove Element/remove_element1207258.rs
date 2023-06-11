// https://leetcode.com/problems/remove-element/solutions/1207258/rust-0ms/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let n: usize = nums.len();
        let mut size: usize = 0;
        for i in 0..n {
            if nums[i] != val {
                nums[size] = nums[i];
                size += 1;
            }
        }
        size as i32
    }
}