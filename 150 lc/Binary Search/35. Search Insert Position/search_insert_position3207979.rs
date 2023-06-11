// https://leetcode.com/problems/search-insert-position/solutions/3207979/rust-binary-search-solution/
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() as i32);

        while l < r {
            let mid = l + (r - l) / 2;

            if nums[mid as usize] == target {
                return mid;
            }
            if nums[mid as usize] > target {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        l
    }
}