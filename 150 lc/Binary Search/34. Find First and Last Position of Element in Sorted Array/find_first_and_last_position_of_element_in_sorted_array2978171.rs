// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/solutions/2978171/rust-binary-search-solution/
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }

        let first = Self::find_first(&nums, target);
        let second = Self::find_last(&nums, target);

        vec![first, second]
    }

    fn find_first(nums: &Vec<i32>, target: i32) -> i32 {
        let (mut start, mut end) = (0, nums.len() - 1);

        while start + 1 < end {
            let mid = start + (end - start) / 2;

            if nums[mid] == target {
                end = mid;
            } else if nums[mid] < target {
                start = mid;
            } else {
                end = mid;
            }
        }

        if nums[start] == target {
            return start as i32;
        }
        if nums[end] == target {
            return end as i32;
        }
        -1
    }

    fn find_last(nums: &Vec<i32>, target: i32) -> i32 {
        let (mut start, mut end) = (0, nums.len() - 1);

        while start + 1 < end {
            let mid = start + (end - start) / 2;

            if nums[mid] == target {
                start = mid;
            } else if nums[mid] < target {
                start = mid;
            } else {
                end = mid;
            }
        }

        if nums[end] == target {
            return end as i32;
        }
        if nums[start] == target {
            return start as i32;
        }
        -1
    }
}