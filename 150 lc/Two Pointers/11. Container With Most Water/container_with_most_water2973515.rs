// https://leetcode.com/problems/container-with-most-water/solutions/2973515/2-pointer-o-n-c-java-python-c-rust-javascript-c/
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len()-1;
        let mut out = 0;
        loop {
            if left >= right {
                break;
            }
            let mut area = std::cmp::min(height[left], height[right]) * ((right-left) as i32);
            out = std::cmp::max(out, area);
            if height[left] < height[right]{
                left += 1;
            }
            else {
                right -= 1;
            }
        }
        out
    }
}