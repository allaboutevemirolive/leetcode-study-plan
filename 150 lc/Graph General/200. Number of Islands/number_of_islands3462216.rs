// https://leetcode.com/problems/number-of-islands/solutions/3462216/rust-faster-than-many-user-s-commits/
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }

        let mut total = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                total += helper(&mut grid, i as i32, j as i32);
            }
        }
        
        total
    }
}

fn helper(grid: &mut Vec<Vec<char>>, i: i32, j: i32) -> i32 {
    if i < 0 || j < 0 || i as usize >= grid.len() || j as usize >= grid[0].len() || grid[i as usize][j as usize] != '1' {
        return 0;
    }

    grid[i as usize][j as usize] = ' ';
    helper(grid, i + 1, j);
    helper(grid, i - 1, j);
    helper(grid, i, j + 1);
    helper(grid, i, j - 1);
    
    return 1;
}