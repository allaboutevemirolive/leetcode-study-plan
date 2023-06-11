// https://leetcode.com/problems/number-of-islands/solutions/3289917/rust-3-approaches/
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        fn dfs1(grid: Vec<Vec<char>>) -> i32 {
            const DIR: [(i32, i32); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
            fn dfs(coord: (i32, i32), grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) {
                if coord.0 < 0
                    || coord.0 >= grid.len() as i32
                    || coord.1 < 0
                    || coord.1 >= grid[0].len() as i32
                    || visited[coord.0 as usize][coord.1 as usize]
                    || grid[coord.0 as usize][coord.1 as usize] == '0'
                {
                    return;
                }
                visited[coord.0 as usize][coord.1 as usize] = true;
                for d in DIR {
                    dfs((coord.0 + d.0, coord.1 + d.1), grid, visited);
                }
            }
            let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
            let mut components_count = 0;
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if grid[i][j] == '1' && !visited[i][j] {
                        dfs((i as i32, j as i32), &grid, &mut visited);
                        components_count += 1;
                    }
                }
            }
            components_count
        }
        fn dfs2(mut grid: Vec<Vec<char>>) -> i32 {
            const DIR: [(i32, i32); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
            fn dfs(coord: (i32, i32), grid: &mut Vec<Vec<char>>) -> bool {
                if coord.0 < 0
                    || coord.0 >= grid.len() as i32
                    || coord.1 < 0
                    || coord.1 >= grid[0].len() as i32
                    || grid[coord.0 as usize][coord.1 as usize] == '0'
                {
                    return false;
                }
                grid[coord.0 as usize][coord.1 as usize] = '0';
                for d in DIR {
                    dfs((coord.0 + d.0, coord.1 + d.1), grid);
                }
                true
            }
            let mut ans = 0;
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if dfs((i as i32, j as i32), &mut grid) {
                        ans += 1;
                    }
                }
            }
            ans
        }
        fn iterative(mut grid: Vec<Vec<char>>) -> i32 {
            let mut ans = 0;
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if grid[i][j] == '0' {
                        continue;
                    }
                    let mut stack = vec![];
                    stack.push((i, j));
                    while let Some((i, j)) = stack.pop() {
                        if grid[i][j] == '0' {
                            continue;
                        }
                        grid[i][j] = '0';
                        if i > 0 {
                            stack.push((i - 1, j));
                        }
                        if i < grid.len() - 1 {
                            stack.push((i + 1, j));
                        }
                        if j > 0 {
                            stack.push((i, j - 1));
                        }
                        if j < grid[0].len() - 1 {
                            stack.push((i, j + 1));
                        }
                    }
                    ans += 1;
                }
            }
            ans
        }
        iterative(grid) 
    }
}