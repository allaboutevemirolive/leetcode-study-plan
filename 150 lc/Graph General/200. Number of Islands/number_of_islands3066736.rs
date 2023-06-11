// https://leetcode.com/problems/number-of-islands/solutions/3066736/dfs-in-rust-code/
impl Solution {
    fn dfs(grid: &mut Vec<Vec<char>>, i:usize, j:usize){
         if(i < 0 || j < 0 || i >= grid.len() || j >= grid[0].len()){
             return;
         }
         if(grid[i][j] != '1'){
             return;
         }
         grid[i][j] = '#';
         Self::dfs(grid, i+1,j);
         Self::dfs(grid, i-1,j);
         Self::dfs(grid, i,j+1);
         Self::dfs(grid, i,j-1);
    }
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid2 = grid.clone();
        if(grid2.is_empty()){
            return 0;
        }
        let mut count = 0;
        for i in 0..grid2.len(){
            for j in 0..grid2[0].len(){
                if(grid2[i][j] == '1'){
                    Self::dfs(&mut grid2, i, j);
                    count+=1;
                }
            }
        }
        return count;
    }
}