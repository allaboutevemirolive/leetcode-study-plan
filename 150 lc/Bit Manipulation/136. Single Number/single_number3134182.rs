// https://leetcode.com/problems/single-number/solutions/3134182/rust-o-1-space-solution/
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ans : i32 = 0;

        for i in nums {
            ans ^= i;
        }
        ans
    }
}