// https://leetcode.com/problems/single-number/solutions/3060060/rust-using-bitwise-xor/
impl Solution {
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        let mut res = 0;
        nums.into_iter().for_each(|n| res ^= n);
        res
    }
}