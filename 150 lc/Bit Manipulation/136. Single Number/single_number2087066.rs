// https://leetcode.com/problems/single-number/solutions/2087066/rust-clean-solution-beat-100-runtime/
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // Definition:
        // XOR: Exclusive OR, meaning only different bits are considered 1
        // ie: a = "0 0 1 1" is xord with b = "0 0 0 1"; result = "0 0 1 0"
        
        // Concepts:
        // 1. If XOR is taken of any number a with 0 then the result is a (ie: a^0=a)
        // 2. If xor is taken of any number a with itself then result is zero(a^a = 0)
        
        // Now the idea is to maintain a result variable which is initilally zero
        // taverse the array and take xor with the ith number, and return the result
        // at the end. It is assumed that each number which is appeared twice 
        // will cancel each other, except the number is appeared only once.
        let mut result = 0;
        
        // Traverese the array and take the xor with the result
        for i in 0..nums.len() {
            result = result ^ nums[i];
        }
        
        // Return the result, the number which is appeared only once 
        // will be the result
        return result
    }
}