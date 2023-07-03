// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/solutions/2331068/rust-0ms-100-binary-search-short/
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = nums.len()-1;
        let mut mid: usize;
        while low < high {
            mid = low + (high - low) / 2;
            if nums[mid] > nums[high] {
                low = mid +1;
            } else {
                high = mid;
            }
        }
        nums[low]
    }
}