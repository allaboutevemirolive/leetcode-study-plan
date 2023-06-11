// https://leetcode.com/problems/number-of-islands/solutions/2502200/rust-recursive-iterative-dfs-solutions/
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    count += 1;
                    Self::dfs(&mut grid, i, j);
                }
            }
        }
        count        
    }
    fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if i >= grid.len() || j >= grid[0].len() || grid[i][j] != '1' {
            return;
        }
        grid[i][j] = '0';
        Self::dfs(grid, i + 1, j);
        Self::dfs(grid, i - 1, j);
        Self::dfs(grid, i, j + 1);
        Self::dfs(grid, i, j - 1);
    }
}