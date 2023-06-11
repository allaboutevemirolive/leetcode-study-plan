// https://leetcode.com/problems/remove-element/solutions/3173282/rust-2-pointer/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let (mut start, mut end) = (0, nums.len());

        while start < end {
            if nums[start] == val {
                end -= 1;
                if nums[end] != val {
                    nums[start] = nums[end];
                } else { continue; }
            }
            start += 1;
        }
        end as i32
    }
}