// https://leetcode.com/problems/unique-paths-ii/solutions/1181183/rust-0ms-o-n-m-time-o-1-space/
impl Solution {
    pub fn unique_paths_with_obstacles(mut grid: Vec<Vec<i32>>) -> i32 {
        let (width, height) = (grid[0].len(), grid.len());
        for y in 0..height {
            for x in 0..width {
                grid[y][x] = match grid[y][x] {
                    0 if (x, y) == (0, 0) => 1,
                    0 => {
                        let up = if y > 0 { grid[y - 1][x] } else { 0 };
                        let left = if x > 0 { grid[y][x - 1] } else { 0 };
                        up + left
                    }
                    _ => 0,
                }
            }
        }

        grid[height - 1][width - 1]
    }
}