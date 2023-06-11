// https://leetcode.com/problems/sqrtx/solutions/2823894/binary-search-in-the-rust/
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        // use i64 to avoid edge cases
        let mut left: i64 = 0;
        let mut right: i64 = x as i64;
        while left < right {
            // because we judge left < right in every iteration
            // and we write `left = mid` in the else block
            // if we didn't add 1 here, we will always get the lower bound
            // , there is no change to exit the loop
            let mid = left + (right - left + 1) / 2;
            // every time we will half the search interval
            if mid > x as i64 / mid {
                right = mid - 1;
            } else {
                left = mid;
            }
        }

        // left will be equall to right here
        // , we just return left
        left as i32
    }
}
