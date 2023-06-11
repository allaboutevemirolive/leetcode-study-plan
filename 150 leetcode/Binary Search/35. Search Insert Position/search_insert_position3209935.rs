// https://leetcode.com/problems/search-insert-position/solutions/3209935/rust-one-line/
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|val| val) as i32
    }
}