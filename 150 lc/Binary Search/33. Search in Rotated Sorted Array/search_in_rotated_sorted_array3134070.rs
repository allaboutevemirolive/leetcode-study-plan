// https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/3134070/rust-solution-2-binary-search/
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let pivot = Solution::search_pivot(&nums);
        println!("pivot {}", pivot);
        Solution::search_with_pivot(&nums, target, pivot)
    }

    fn search_with_pivot(nums: &Vec<i32>, target: i32, pivot: usize) -> i32 {
        let to_idx = |idx: usize| (idx + pivot)%nums.len();
        let (mut start, mut end) = (0, nums.len());
        while start < end {
            let mid = (start + end)/2;
            let m = nums[to_idx(mid)];
            if m == target {
                return to_idx(mid) as i32
            } else if m < target {
                start = mid + 1
            } else {
                end = mid
            }
        }
        -1
    }

    fn search_pivot(nums: &Vec<i32>) -> usize {
        let (mut start, mut end) = (0, nums.len());
        while start < end {
            let mid = (start + end)/2;
            if nums[mid] >= nums[0] {
                start = mid + 1;
            } else {
                end = mid;
            }
        }
        end
    }
}