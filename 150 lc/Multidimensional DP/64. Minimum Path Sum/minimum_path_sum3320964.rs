// https://leetcode.com/problems/minimum-path-sum/solutions/3320964/rust-in-place-dp-solution/
use std::cmp::min;
impl Solution {
  pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
    for v in 1..grid.len() {
      grid[v][0]+=grid[v-1][0];
    }
    for h in 1..grid[0].len() {
      grid[0][h]+=grid[0][h-1];
    }
    for i in 1..grid.len() {
      for j in 1..grid[0].len() {
        grid[i][j]+=min(grid[i-1][j],grid[i][j-1]);
      }
    }
    grid[grid.len()-1][grid[0].len()-1]
  }
}