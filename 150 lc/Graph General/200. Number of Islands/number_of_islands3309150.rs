// https://leetcode.com/problems/number-of-islands/solutions/3309150/rust-dfs-simple-solution/
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut islands = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j]=='1' {
                    islands+=1;
                //gets rid of all neighboring 1s
                    Self::dfs(&mut grid,i,j);
                }
            }
        }
        return islands;
    }
    fn dfs(grid:&mut Vec<Vec<char>>, i:usize, j:usize ){
        if i<0 || j<0 || i>=grid.len() || j>=grid[0].len() || grid[i][j] != '1' {
            return;
        }
        grid[i][j]='0';
        Self::dfs(grid, i+1, j);
        Self::dfs(grid,i-1,j);
        Self::dfs(grid,i,j+1);
        Self::dfs(grid,i,j-1);
    }
}