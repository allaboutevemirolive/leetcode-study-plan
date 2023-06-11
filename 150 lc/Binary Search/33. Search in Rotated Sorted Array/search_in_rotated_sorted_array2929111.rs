// https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/2929111/0ms-rust-solution/
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        let (mut l, mut r) = (0, len - 1);

        while l <= r {
            while l < len && nums[l] > target {
                l += 1;
            }

            while r > 0 && nums[r] < target {
                r -= 1;
            }

            if l < len && nums[l] == target {
                return l as i32;
            } else {
                l += 1;
            }

            if nums[r] == target {
                return r as i32;
            } else if r > 0 {
                r -= 1;
            }
        }

        -1
    }
}