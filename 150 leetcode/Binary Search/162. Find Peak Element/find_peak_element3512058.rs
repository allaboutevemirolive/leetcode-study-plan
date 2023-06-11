// https://leetcode.com/problems/find-peak-element/solutions/3512058/rust-solution/
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] > nums[mid + 1] {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo as i32
    }
}