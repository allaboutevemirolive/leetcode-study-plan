// https://leetcode.com/problems/container-with-most-water/solutions/3068069/container-with-most-water-using-rust/
use std::cmp;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left_idx = 0;
        let mut right_idx = height.len() as i32 - 1;
        
        while left_idx < right_idx {
            let left_num = height[left_idx as usize];
            let right_num = height[right_idx as usize];
            let min_height = cmp::min(left_num, right_num);
            max_area = cmp::max(max_area, min_height * (right_idx - left_idx));

            if left_num <= right_num {
                left_idx += 1;
            }
            else {
                right_idx -= 1;
            }
        }
        max_area
    }
}