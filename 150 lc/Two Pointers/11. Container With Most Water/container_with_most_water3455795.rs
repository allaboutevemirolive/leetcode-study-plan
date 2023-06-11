// https://leetcode.com/problems/container-with-most-water/solutions/3455795/o-n-solution-with-rust/
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut n = height.len();
        let mut l=0;
        let mut r=n-1;
        let mut ans=0;
        while l < r {
            ans = std::cmp::max(ans,
            (std::cmp::min(height[l as usize],height[r as usize])*(r as i32 - l as i32)));
            if height[l as usize]<height[r as usize] {
                l+=1;
            } else {
                r-=1;
            }
        }
        return ans;
    }
}