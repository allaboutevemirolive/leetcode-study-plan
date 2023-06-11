// https://leetcode.com/problems/container-with-most-water/solutions/3162827/rust-o-n-solution-easy-explanation/
use std::cmp::{max, min, Ordering};
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut lower = 0;
        let mut upper = height.len() - 1;
        let mut cap = 0;
        
        while lower < upper {
            cap = max(cap, min(height[lower], height[upper])*(upper as i32 - lower as i32));
            match (height[lower]).cmp(&height[upper]){
                Ordering::Less => lower += 1,
                _ => upper -= 1,
                
            }
        }
        cap
    }
}