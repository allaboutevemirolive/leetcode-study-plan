// https://leetcode.com/problems/spiral-matrix/solutions/3561608/rust-simple-directions-matrix-bounds/
use Direction::*;

#[derive(Copy, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return Vec::new();
        }

        let mut v = Vec::with_capacity(matrix.len() * matrix[0].len());
        
        let mut left = 0;
        let mut top = 0;
        let mut right = matrix[0].len() - 1;
        let mut bottom = matrix.len() - 1;

        let mut direction = Right;
        let mut row = 0;
        // start at -1 because we 
        // move in the current 
        // direction before pushing
        let mut col: isize = -1;
        
        for _ in 0..v.capacity() {
            match direction {
                Right => {
                    col += 1;
                    if col as usize == right {
                        top += 1;
                        direction = Down;
                    }
                }
                Down => {
                    row += 1;
                    if row == bottom {
                        right -= 1;
                        direction = Left;
                    }
                }
                Left => {
                    col -= 1;
                    if col == left {
                        bottom -= 1;
                        direction = Up;
                    }
                }
                Up => {
                    row -= 1;
                    if row == top {
                        left += 1;
                        direction = Right;
                    }
                }
            }
            v.push(matrix[row][col as usize]);     
        }
        v
    }
}