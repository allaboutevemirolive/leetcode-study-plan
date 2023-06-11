// https://leetcode.com/problems/search-insert-position/solutions/3209602/rust-binary-search/
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut right = nums.len();
        let mut left = 0;

        while left < right {
            let mid = (left + right) / 2;
            let num = nums[mid];
            if num == target {
                return mid as i32;
            } else if target > num {
                left = mid + 1;
            } else if target < num {
                right = mid;
            }
        }
        return left as i32;
    }
}