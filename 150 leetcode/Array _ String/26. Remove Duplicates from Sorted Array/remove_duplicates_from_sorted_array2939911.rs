// https://leetcode.com/problems/remove-duplicates-from-sorted-array/solutions/2939911/0ms-rust-solution-with-two-pointers/
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let (mut slow, mut fast) = (0, 0);

        while fast < nums.len() {
            if nums[fast] != nums[slow] {
                slow += 1;
                nums[slow] = nums[fast];
            }

            fast += 1;
        }

        slow as i32 + 1
    }
}