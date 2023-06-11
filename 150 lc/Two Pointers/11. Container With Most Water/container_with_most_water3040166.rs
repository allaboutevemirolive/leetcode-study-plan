// https://leetcode.com/problems/container-with-most-water/solutions/3040166/rust-simple-solution/
use std::cmp::Ordering;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let mut max_vol = 0;

        while left < right {
            let (h1, h2) = (height[left], height[right]);

            max_vol = ((right - left) as i32 * match h1.cmp(&h2) {
                Ordering::Less => { left += 1; h1 },
                _ => { right -= 1; h2 }
            }).max(max_vol);
        }
        max_vol
    }
}