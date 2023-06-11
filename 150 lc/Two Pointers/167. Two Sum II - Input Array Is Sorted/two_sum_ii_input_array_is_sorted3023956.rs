// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/solutions/3023956/rust-imperative-functional-style-solutions/
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l_idx, mut r_idx) = (0, numbers.len() - 1);
        loop {
            match numbers[l_idx] + numbers[r_idx] {
                t if t < target => l_idx += 1,
                t if t > target => r_idx -= 1,
                _ => break vec![l_idx as i32 + 1, r_idx as i32 + 1]
            }
        }
    }
}