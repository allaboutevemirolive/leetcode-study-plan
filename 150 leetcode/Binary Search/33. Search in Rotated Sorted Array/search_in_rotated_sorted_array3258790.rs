// https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/3258790/rust-0ms-recursive-divide-and-conquer-using-two-match-expressions/
fn _search(nums: &[i32], target: i32) -> i32 {
    match *nums {
        // Trivial cases:
        [] => -1,
        [x] if x == target => 0,
        [_] => -1,
        // Definetly not rotated:
        [a, .., b] if a < b => {
            nums.binary_search(&target).map_or(-1, |res| res as i32)
        }
        // Otherwise, divide and conquer:
        _ => match (
            _search(&nums[0..nums.len() / 2], target),
            _search(&nums[nums.len() / 2..], target),
        ) {
            (-1, -1) => -1,
            (n, -1) => n,
            (-1, n) => n + (nums.len() / 2) as i32,
            _ => unreachable!(),
        },
    }
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        _search(&*nums, target)
    }
}