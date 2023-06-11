// https://leetcode.com/problems/search-insert-position/solutions/3209586/rust-2-binary-search-solutions-with-a-bonus/
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|i| i) as _
    }
}