// https://leetcode.com/problems/single-number/solutions/3226611/rust-simple-bitwise-xor/
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut solution: i32 = 0;
        for num in nums {
            solution ^= num;
        }
        solution
    }
}