// https://leetcode.com/problems/maximum-subarray/solutions/2902421/chain-rust-python-swift-4-lines/
use std::cmp::max;

impl Solution {
    pub fn max_sub_array(A: Vec<i32>) -> i32 {
        let (mut x, mut y) = (0, i32::MIN);
        A.iter().for_each(|&i| {
            x = max(x + i, i);
            y = max(y, x);
        }); y
    }
}