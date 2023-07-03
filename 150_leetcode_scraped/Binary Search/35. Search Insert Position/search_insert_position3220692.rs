// https://leetcode.com/problems/search-insert-position/solutions/3220692/rust-cannot-get-more-shorter-than-this/
use std::convert::TryInto;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }
}