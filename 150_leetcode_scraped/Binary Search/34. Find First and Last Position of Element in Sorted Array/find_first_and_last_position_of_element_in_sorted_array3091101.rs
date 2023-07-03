// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/solutions/3091101/rust-2-approaches-binary-searches-stdlib-unit-tests/
use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        match nums.binary_search(&target) {
            Err(_) => return vec![-1, -1],
            Ok(_) => {
                return vec![
                    Solution::binary_search_left(&nums, target),
                    Solution::binary_search_right(&nums, target),
                ];
            }
        }
    }

    #[cfg(use_stdlib)] // don't compile this without Rust's `#define use_stdlib`
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        match nums.binary_search(&target) {
            Err(_) => return vec![-1, -1],
            Ok(_) => {
                let (p1, p2) = ( // <--- tricky part
                    nums.partition_point(|x| x < &target),
                    nums.partition_point(|x| x <= &target), // means leave behind all values less than or equal, 
                                            // i.e. stop at first greater value. 
                                            // Returns length of the vector is there are no greater values (which is correct for sorted vec)
                );

                vec![p1 as i32, (p2 - 1) as i32]
            }
        }
    }

    pub fn binary_search_right<T>(nums: &Vec<T>, target: T) -> i32
    where
        T: std::cmp::Ord,
    {
        let (mut l, mut h) = (0 as i32, nums.len() as i32 - 1 as i32);
        let mut res = h;
        while l <= h {
            let m = l + (h - l) / 2;
            match target.cmp(&nums[m as usize]) {
                Greater => l = m + 1,
                Less => h = m - 1,
                Equal => {
                    l = m + 1;
                    res = m;
                }
            }
        }
        res
    }
    pub fn binary_search_left<T>(nums: &Vec<T>, target: T) -> i32
    where
        T: std::cmp::Ord,
    {
        let (mut l, mut h) = (0 as i32, nums.len() as i32 - 1 as i32);
        let mut res = h;
        while l <= h {
            let m = l + (h - l) / 2;
            match target.cmp(&nums[m as usize]) {
                Greater => l = m + 1,
                Less => h = m - 1,
                Equal => {
                    h = m - 1;
                    res = m;
                }
            }
        }
        res
    }
}

#[cfg(test)] // cargo test --no-fail-fast
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4],
            "1"
        );

        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 28),
            vec![-1, -1],
            "2"
        );

        assert_eq!(
            Solution::search_range(vec![1, 1, 1, 1, 1, 1, 1, 1], 1),
            vec![0, 7],
            "3"
        );
    }
}

