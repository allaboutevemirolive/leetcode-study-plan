// https://leetcode.com/problems/search-insert-position/solutions/3406189/rust-binary-search/
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(n) => n as i32,
            Err(n) => n as i32
        }
    }
}