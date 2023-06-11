// https://leetcode.com/problems/remove-duplicates-from-sorted-array/solutions/3523195/rust-implementation/
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut current: usize = 0;
        let mut next: usize = 1;
        let size = nums.len();
        if size <= 1 {
            return size as i32;
        }
        while next < size {
            // compare next with current, if they are the same, move next forward
            if nums[current] == nums[next] {
                next += 1;
            } else {
                // else, the nums[next] is a new value, put it to nums[current+1], forward current and next
                nums[current + 1] = nums[next];
                current += 1;
                next += 1;
            }
        }
        (current + 1) as i32
    }
}