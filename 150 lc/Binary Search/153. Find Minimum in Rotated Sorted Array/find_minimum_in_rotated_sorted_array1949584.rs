// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/solutions/1949584/rust-binary-search/
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            if nums[lo] < nums[hi] {
                return nums[lo];
            }
            
            let mid = lo + (hi - lo) / 2;
            // lo and mid and be same
            // [2, 1]
            //  ^
            if nums[lo] <= nums[mid] {
                lo = mid + 1;
            } else {
                hi = mid
            }
        }
        
        nums[lo] as i32
    }
}