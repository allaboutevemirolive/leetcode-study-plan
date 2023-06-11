// https://leetcode.com/problems/container-with-most-water/solutions/3566193/rust-two-pointer/
// Calculates the volume given left and right index
fn vol(height: &Vec<i32>, p0: usize, p1: usize) -> i32 {
    std::cmp::min(height[p0], height[p1]) * (p1 as i32 - p0 as i32)
}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut p0 = 0; // left
        let mut p1 = height.len() - 1; // right
        let mut max_vol = vol(&height, p0, p1);

        while p0 < p1 {
            max_vol = std::cmp::max(max_vol, vol(&height, p0, p1));

            if height[p0] < height[p1] {
                p0 += 1;
            } else {
                p1 -= 1;
            }
        }

        max_vol
    }
}
