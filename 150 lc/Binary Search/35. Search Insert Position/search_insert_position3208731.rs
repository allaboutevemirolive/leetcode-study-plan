// https://leetcode.com/problems/search-insert-position/solutions/3208731/o-log-n-rust-solution-using-binary-search/
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo: usize = 0;
        let mut hi: usize = nums.len() - 1;
        let mut mid: usize = 0;
        while (lo <= hi) {
            mid = lo + (hi - lo) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[mid] > target {
                if mid == 0 { break } // since indices are usize
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        lo as i32
    }
}