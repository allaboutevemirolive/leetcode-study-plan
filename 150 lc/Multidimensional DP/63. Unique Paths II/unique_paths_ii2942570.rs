// https://leetcode.com/problems/unique-paths-ii/solutions/2942570/1ms-solution-in-rust-using-tabulation/
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut table: Vec<Vec<Option<i32>>> = Vec::new();
        for _i in 0..obstacle_grid.len() + 1 {
            let inner_table: Vec<Option<i32>> = Vec::new();
            table.push(inner_table.clone());
            for _j in 0..obstacle_grid[0].len() + 1 {
                table[_i].push(None);
            }
        }
        for i in 0..obstacle_grid.len() + 1 {
            table[i][0] = Some(0);
        }
        for j in 0..obstacle_grid[0].len() + 1 {
            table[0][j] = Some(0);
        }
        for i in 1..obstacle_grid.len() + 1 {
            for j in 1..obstacle_grid[0].len() + 1 {
                if obstacle_grid[i - 1][j - 1] != 1 {
                    table[i][j] = Some(0);
                }
            }
        }
        if obstacle_grid[0][0] != 1 {
            table[1][1] = Some(1);
        } else {
            return 0;
        }
        for i in 1..obstacle_grid.len() + 1 {
            for j in 1..obstacle_grid[0].len() + 1 {
                if i + 1 <= obstacle_grid.len() {
                    if !table[i + 1][j].is_none() && !table[i][j].is_none() {
                        table[i + 1][j] = Some(table[i + 1][j].unwrap() + table[i][j].unwrap());
                    }
                }
                if j + 1 <= obstacle_grid[0].len() {
                    if !table[i][j + 1].is_none() && !table[i][j].is_none() {
                        table[i][j + 1] = Some(table[i][j + 1].unwrap() + table[i][j].unwrap());
                    }
                }
            }
        }
        if table[m][n].is_none() {
            0
        } else {
            table[m][n].unwrap()
        }        
    }
}