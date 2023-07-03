// https://leetcode.com/problems/remove-element/solutions/2663443/two-pointers-in-rust/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        let mut j = 0;

        while j < nums.len() {
            if nums[j] != val {
                nums[i] = nums[j];
                i += 1;
            }
            j += 1;
        }

        i as i32
    }
}