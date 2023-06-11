// https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/3013105/rust-binary-search-solution/
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut start, mut end) = (0, nums.len() as i32 - 1);

        while start <= end {
            let mid = start + (end - start) / 2;
            
            if nums[mid as usize] == target {
                return mid;
            } else if nums[start as usize] <= nums[mid as usize] {
                if nums[start as usize] <= target && target < nums[mid as usize] {
                    end = mid - 1;
                } else {
                    start = mid + 1;
                }
            } else {
                if nums[mid as usize] < target && target <= nums[end as usize] {
                    start = mid + 1;
                } else {
                    end = mid - 1;
                }
            }
        }
        
        -1
    }
}