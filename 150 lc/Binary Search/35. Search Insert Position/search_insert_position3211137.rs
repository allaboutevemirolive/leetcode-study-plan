// https://leetcode.com/problems/search-insert-position/solutions/3211137/rust-0ms-2-1-mb/
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(v)  => v as i32,
            Err(v) => v as i32
        }
    }
}