// https://leetcode.com/problems/minimum-path-sum/solutions/3345802/rust-2-small-dp-vectors-0ms-2-3mb/
use std::mem::swap;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut scores1 = vec![i32::MAX; m];
        let mut scores2 = vec![i32::MAX; m];

        scores1[0] = grid[0][0];

        for x in 0..n {
            for y in 0..m {
                let s = scores1[y];
                
                if y < m - 1 {
                    scores1[y + 1] = scores1[y + 1].min(s + grid[x][y + 1]);
                }
                if x < n - 1 {
                    scores2[y] = s + grid[x + 1][y];
                }
            }
            swap(&mut scores1, &mut scores2);
        }
        scores2[m - 1]
    }
}