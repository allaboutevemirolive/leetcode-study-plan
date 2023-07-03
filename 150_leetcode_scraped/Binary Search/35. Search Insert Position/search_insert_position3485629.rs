// https://leetcode.com/problems/search-insert-position/solutions/3485629/rust-one-line-1-ms-2-1-mb/
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    nums.binary_search_by(|index| index.cmp(&target)).unwrap_or_else(|e| e) as i32
}