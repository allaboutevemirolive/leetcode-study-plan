// https://leetcode.com/problems/number-of-islands/solutions/3115552/rust-15ms-9-2-mb/
use std::collections::HashSet;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut count = 0;

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == '1' && !visited.contains(&(row, col)) {
                    count += 1;
                    Self::bfs(row, col, &grid, &mut visited);
                }
            }
        }

        count
    }

    fn valid_moves(row: usize, col: usize, grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
        let moves: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut valid_moves = vec![];

        for mv in moves {
            let (x0, y0) = mv;
            let (x, y) = (x0 + row as i32, y0 + col as i32);

            if x >= 0
                && x < grid.len() as i32
                && y >= 0
                && y < grid[0].len() as i32
                && grid[x as usize][y as usize] == '1'
            {
                valid_moves.push((x as usize, y as usize));
            }
        }

        valid_moves
    }

    fn bfs(row: usize, col: usize, grid: &Vec<Vec<char>>, visited: &mut HashSet<(usize, usize)>) {
        let mut stack: Vec<(usize, usize)> = vec![(row, col)];
        visited.insert((row, col));

        while stack.len() > 0 {
            let (x, y) = stack.pop().unwrap();
            let valid_moves = Self::valid_moves(x, y, grid);
            for mv in valid_moves {
                if !visited.contains(&mv) {
                    visited.insert(mv);
                    stack.push(mv);
                }
            }
        }
    }
}