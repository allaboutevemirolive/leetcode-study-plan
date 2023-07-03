// https://leetcode.com/problems/container-with-most-water/solutions/3070819/rust-two-pointer-advance-pointer-on-side-of-smaller-height/
use std::cmp::{max, Ordering};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, height.len() -1);
        let mut max_water = 0;
        while l < r {
            match height[l].cmp(&height[r]) {
                Ordering::Less => {
                    max_water = max((r - l) * height[l] as usize, max_water);
                    l += 1;
                }
                _ => {
                    max_water = max((r - l) * height[r] as usize, max_water);
                    r -= 1;
                }
            }
        }
        max_water as i32
    }
}