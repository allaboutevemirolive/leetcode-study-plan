// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/solutions/3277576/rust-nice-binary-search-reusable-code-solution/
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
         if !nums.contains(&target) {
            return vec![-1,-1]
        }
        let comparisonl = |l:i32, r:i32| -> bool {
            l < r
        };
        let comparisonr = |l:i32, r:i32| -> bool {
            l <= r
        };

        vec![Self::binary_search(&nums, target, &comparisonl), Self::binary_search(&nums, target, &comparisonr)-1]
    }
    pub fn binary_search(nums: &Vec<i32>, target: i32, comparison: &dyn Fn(i32, i32) -> bool) -> i32 {
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let mid = (l + r)/2;
            if comparison(nums[mid], target) {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l as i32
    }
}