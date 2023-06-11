// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/solutions/3561624/rust-binary-search-favouring-left-and-right-steps-with-generic-closure/
use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Equal, Less};

impl Solution {
    pub fn search_range(nums: Vec<i32>, t: i32) -> Vec<i32> {
        // check if we can find the value at all
        if Self::binary_search(&nums, |&x| x.cmp(&t)).is_err() {
            return vec![-1, -1] 
        }

        // note that from below we use .unwrap_err(), since the closures never return
        // Ordering::Equal, we can safely always expect the Err variant of the Result

        // right step favouring binary search for t:
        let u = Self::binary_search(&nums, |x| if x>&t {Greater} else {Less}).unwrap_err();

        // left step favouring binary search for t:
        let l = Self::binary_search(&nums, |x| if x<&t {Less} else {Greater}).unwrap_err();

        vec![l, u-1] // u-1 to make the range [open, open]
    }

    /// Re-implementing the binary_search_by
    /// from core https://doc.rust-lang.org/src/core/slice/mod.rs.html#2502-2504
    fn binary_search<'a, F>(nums: &'a Vec<i32>, f: F) -> Result<i32, i32>
    where F: Fn(&'a i32) -> Ordering {
        let (mut l, mut u) = (0, nums.len());
        while l < u {
            let mid = (l+u)/2;
            match f(&nums[mid]) { // idiomatic match ignoring speed benefits from if...
                Greater => {u = mid;}
                Less => {l = mid+1;}
                Equal => return Ok(mid as i32)
            }
        }
        Err(l as i32)
    }
}