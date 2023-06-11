// https://leetcode.com/problems/single-number/solutions/2915519/rust-o-n/
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for c in nums {
            res = res ^ c;
        }
        res
    }
}