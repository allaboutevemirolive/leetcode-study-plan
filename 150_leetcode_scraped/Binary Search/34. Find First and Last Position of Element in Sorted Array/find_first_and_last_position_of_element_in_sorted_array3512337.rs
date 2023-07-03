// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/solutions/3512337/rust-solution/
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        fn search(nums: &Vec<i32>, target: i32, first: bool) -> i32 {
            let mut ans = -1;
            let (mut lo, mut hi) = (0, nums.len() as i32 - 1);
            while lo <= hi {
                let mid = (lo + hi) / 2;

                if first {
                    if nums[mid as usize] >= target {
                        hi = mid - 1;
                    } else {
                        lo = mid + 1;
                    }
                } else {
                    if nums[mid as usize] <= target {
                        lo = mid + 1;
                    } else {
                        hi = mid - 1;
                    }
                }

                if nums[mid as usize] == target {
                    ans = mid;
                }
            }
            ans
        }

        vec![search(&nums, target, true), search(&nums, target, false)]
    }
}