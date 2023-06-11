// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/solutions/3220131/rust-binary-search-partition-point/
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut low, mut high) = (0 , nums.len() as i32 - 1);
    let mut left_idx = -1;
    let mut right_idx = -1;
    while low <= high {
        let mid = (low + high)  / 2;
        if nums[mid as usize] == target {
            left_idx = mid as _;
            high = mid - 1;
        } else if  nums[mid as usize] < target{
             low = mid + 1;
        } else {
            high = mid - 1;
        }

    }
    high = nums.len() as i32 -1;
    while low <= high  {
        let mid = (low + high)  / 2;
        if nums[mid as usize] == target {
            right_idx = mid as _;
            low = mid + 1;
        } else if  nums[mid as usize] < target{
             low = mid + 1;
        } else {
            high = mid - 1;
        }

    }
    vec![left_idx, right_idx]
}