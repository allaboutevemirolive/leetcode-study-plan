// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/solutions/2764374/iterative-binary-search-rust-solution/
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        let mut res = nums[0];

        while lo <= hi {
            if nums[lo] < nums[hi] {
                res = res.min(nums[lo]); 
                break
            }

            let mid = lo + (hi - lo) / 2;

            if nums[lo] <= nums[mid] {
                lo = mid + 1
            }
            else { hi = mid - 1 }
            
            res = res.min(nums[mid]);
        }
        res
    }
}