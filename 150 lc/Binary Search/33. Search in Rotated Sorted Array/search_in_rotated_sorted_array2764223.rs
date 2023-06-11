// https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/2764223/iterative-binary-search-rust-solution/
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);

        while lo <= hi {
            //find the midpoint
            //***Note:*** finding the midpoint this way
            //avoids overflow if we have extremely large numbers
            let mid = lo + (hi - lo) / 2;

            if target == nums[mid] {
                return mid as i32;
            }

            //if the lomost number is less than the midpoint
            //we've found a slice of the lo sorted portion
            match nums[lo] <= nums[mid] {
                true => {
                    //if the target does not exist within our range
                    if target > nums[mid] || target < nums[lo] {
                        lo = mid + 1
                    }
                    //the number must be within our range if it exists
                    else {
                        hi = mid - 1
                    }
                }
                //our pointers found the hi sorted portion
                false => {
                    //if the target does not exist within our range
                    if target < nums[mid] || target > nums[hi] {
                        hi = mid - 1
                    }
                    //the number must be within our range if it exists
                    else {
                        lo = mid + 1
                    }
                }
            }
        }
        -1
    }
}