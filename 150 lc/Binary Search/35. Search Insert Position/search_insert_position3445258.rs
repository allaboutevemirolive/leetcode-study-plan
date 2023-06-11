// https://leetcode.com/problems/search-insert-position/solutions/3445258/rust-solution-recursive-binary-search/
impl Solution {
    fn binary_search(nums: &[i32], target: i32, start_idx: usize, end_idx: usize) -> i32 {
        if nums.len() == 1 {
            if nums[0] == target {
                return 0;
            }
            if target >= nums[0] {
                return (start_idx + 1) as i32;
            } else {
                return start_idx as i32;
            }
        }
        let idx = nums.len() / 2;
        let pivot = nums[idx];
        if target > pivot {
            return Self::binary_search(&nums[idx..], target, start_idx + idx, end_idx);
        } else if target < pivot {
            return Self::binary_search(&nums[..idx], target, start_idx, end_idx + idx);
        } else {
            return (start_idx + idx) as i32;
        }
    }

    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Self::binary_search(&nums[..], target, 0, nums.len() - 1)
    }
}
