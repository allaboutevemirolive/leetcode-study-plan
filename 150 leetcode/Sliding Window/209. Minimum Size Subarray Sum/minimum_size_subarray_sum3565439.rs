// https://leetcode.com/problems/minimum-size-subarray-sum/solutions/3565439/rust-sliding-window-illustrated/
use std::cmp::min;

fn first_window(nums: &[i32], t: i32) -> (Option<usize>, i32) {
    let mut sum = 0;
    for (i, el) in nums.iter().enumerate() {
        sum+=el;
        if sum >= t {return (Some(i+1), sum as _)}
    }
    (None, sum as _)
}

impl Solution {
    pub fn min_sub_array_len(t: i32, nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let (r, mut cur) = first_window(&nums[l..], t);
        if r.is_none() {return 0;} // no window > t

        let n = nums.len();
        let mut r = r.unwrap();
        let mut res = r;
        
        for l in (1..n) {
            cur-=nums[l-1];
            // println!("{nums:?}");
            // println!("[{}^--{}^{}]", "   ".repeat(l-1), "---".repeat((r-l).checked_sub(1).unwrap_or(0)), "   ".repeat(n-r));
            while cur < t && r<n {
                cur+=nums[r]; r+=1;
            }
            if cur>=t {
                res = min(res, r-l)
            }
            // println!("");
        }

        res as _
    }
}