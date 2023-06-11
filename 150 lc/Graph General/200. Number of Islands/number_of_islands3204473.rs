// https://leetcode.com/problems/number-of-islands/solutions/3204473/rust-solution-dfs-easy/
impl Solution {

    pub fn solve(grid : &mut Vec<Vec<char>>,i : usize,j : usize) {
        grid[i][j] = '0';

        let dx : [i32;4] = [-1,1,0,0];
        let dy : [i32;4] = [0,0,-1,1];

        for x in 0..4 {
            let ki = dx[x] + i as i32;
            let kj = dy[x] + j as i32;

            if ki < 0 || kj < 0 || ki >= grid.len() as i32 || kj >= grid[0].len() as i32 || grid[ki as usize][kj as usize] == '0' {
                continue;
            }

            Self::solve(grid,ki as usize ,kj as usize );
        }
    }

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut ans : i32 = 0;

        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                if grid[x][y] == '1' {
                    ans += 1;
                    Self::solve(&mut grid,x,y);
                }
            }
        }
        ans
    }
}