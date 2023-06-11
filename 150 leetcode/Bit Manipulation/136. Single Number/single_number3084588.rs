// https://leetcode.com/problems/single-number/solutions/3084588/rust-one-liner-without-fold/
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {

        nums.into_iter().reduce(|a, n| a ^ n).unwrap()
        
    }
}