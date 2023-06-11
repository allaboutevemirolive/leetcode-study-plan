// https://leetcode.com/problems/container-with-most-water/solutions/3598716/rust-simple-two-pointer-approach-o-n-time-complexity/
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut lhs: usize = 0;
        let mut rhs: usize = height.len() - 1;
        let mut max_volumn: i32 = 0;
        let mut current_volumn: i32 = 0;

        while lhs < rhs {
            current_volumn = (rhs - lhs) as i32 * std::cmp::min(height[rhs], height[lhs]);
            max_volumn = std::cmp::max(max_volumn, current_volumn);

            if (height[rhs] < height[lhs]) {
                rhs -= 1;
            } else {
                lhs += 1;
            }
        }
        return max_volumn;
    }
}