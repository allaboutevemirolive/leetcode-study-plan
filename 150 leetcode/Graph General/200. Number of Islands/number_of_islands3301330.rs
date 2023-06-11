// https://leetcode.com/problems/number-of-islands/solutions/3301330/rust-dfs-implementation/
impl Solution {
    fn mark_island(
        grid: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        m: i32,
        n: i32,
        r: i32,
        c: i32,
    ) {
        visited[r as usize][c as usize] = true;

        let delta_row: [i32; 4] = [1, -1, 0, 0];
        let delta_col: [i32; 4] = [0, 0, 1, -1];

        for (dr, dc) in delta_row.into_iter().zip(delta_col.into_iter()) {
            let nr = r + dr;
            let nc = c + dc;
            if nr >= 0
                && nr < n
                && nc >= 0
                && nc < m
                && !visited[nr as usize][nc as usize]
                && grid[nr as usize][nc as usize] == '1'
            {
                Self::mark_island(&grid, visited, m as i32, n as i32, nr, nc)
            }
        }
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let m = grid[0].len();
        let n = grid.len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];

        let mut total_islands = 0;
        for r in 0..n {
            for c in 0..m {
                if !visited[r][c] && grid[r][c] == '1' {
                    total_islands += 1;
                    Self::mark_island(&grid, &mut visited, m as i32, n as i32, r as i32, c as i32);
                }
            }
        }

        total_islands
    }
}