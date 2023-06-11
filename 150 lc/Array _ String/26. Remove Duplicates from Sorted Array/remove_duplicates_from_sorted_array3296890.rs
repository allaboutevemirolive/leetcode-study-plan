// https://leetcode.com/problems/remove-duplicates-from-sorted-array/solutions/3296890/rust-100-solution/
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut write_index = 1;

        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[write_index] = nums[i];
                write_index += 1
            }
        }

        return write_index as i32;
    }
}