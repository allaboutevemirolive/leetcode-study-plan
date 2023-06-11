// https://leetcode.com/problems/container-with-most-water/solutions/3586640/learning-rust-i-did-this-in-5-minutes/
use std::cmp::{max, min};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut water = 0;

        while l < r {
            let min_height = min(height[l], height[r]);
            let width = (r - l) as i32;
            
            water = max(water, width * min_height);

            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        water
    }
}