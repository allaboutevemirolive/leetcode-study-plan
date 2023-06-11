// https://leetcode.com/problems/search-insert-position/solutions/3210552/rust-inbuilt-funtion-use-100-faster/
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(x) => x as i32,
            Err(x) => x as i32,
        }
    }
}