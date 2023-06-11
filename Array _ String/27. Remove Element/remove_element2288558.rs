// https://leetcode.com/problems/remove-element/solutions/2288558/rust/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut cnt = nums.len();
        let mut idx = 0;
        while idx < cnt {
            if nums[idx] == val {
                nums.swap(idx, cnt - 1);
                cnt -= 1;
                continue;
            }
            idx += 1;
        }
        cnt as i32
    }
}