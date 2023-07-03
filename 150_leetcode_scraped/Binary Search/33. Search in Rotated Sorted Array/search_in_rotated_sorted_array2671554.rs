// https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/2671554/delicious-rust-0ms-simply-2-sorted-subarrays/
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut hi = nums.len() - 1;
        let mut lo = 0;
        loop {
			//find inversion point
            let mid = (hi + lo) / 2;
            if mid == lo {
                break;
            }
            if nums[mid] > nums[hi] {
                // look right
                lo = mid
            } else {
                //look left
                hi = mid
            }
        }
		// search sorted subarrays
        if let Ok(idx) = &nums[..lo+1].binary_search(&target) {
            return *idx as i32;
        }
        if let Ok(idx) = &nums[lo+1..].binary_search(&target) {
            return (lo+*idx+1) as i32;
        }
        return -1;
    }
}