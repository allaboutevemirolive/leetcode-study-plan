// https://leetcode.com/problems/triangle/solutions/3023422/rust-dynamic-programming-bottom-up-iteration-in-place-modify-from-leaf/
use std::cmp::min;

impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        for row in (0..(triangle.len()-1)).rev() {
            for col in 0..=row {
                triangle[row][col] += min(triangle[row+1][col], triangle[row+1][col+1]);
            }
        }
        triangle[0][0]
    }
}