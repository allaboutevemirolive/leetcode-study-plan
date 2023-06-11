// https://leetcode.com/problems/container-with-most-water/solutions/3529997/rust-in-5-mins/
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        // optimizing for width and height. height is limited by the max value of the array
        // width is limited by the length of the array
        let max_w = height.len();
        // I'd say we want to do a (left, right) approach
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max = 0;
        while left < right {
            let l = height[left];
            let r = height[right];
            let volume = max.max(l.min(r) * (right as i32 -left as i32));
            max = max.max(volume);
            if l < r {
                left += 1;
            } else if l > r {
                right -= 1;
            } else {
                left += 1;
                right -= 1;
            }
        }
        return max;
    }
}