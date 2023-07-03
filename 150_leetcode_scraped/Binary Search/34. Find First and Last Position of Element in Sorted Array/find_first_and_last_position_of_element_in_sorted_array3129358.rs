// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/solutions/3129358/rust-fast-simple-beats-100/
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        match nums.iter().position(|&x| x == target) {
            Some(l) => match nums.iter().rposition(|&x| x == target) {
                Some(r) => vec![l as _, r as _],
                None => vec![l as _, l as _],
            },
            None => vec![-1, -1],
        }
    }
}