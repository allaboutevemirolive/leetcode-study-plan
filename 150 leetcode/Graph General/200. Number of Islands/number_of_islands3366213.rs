// https://leetcode.com/problems/number-of-islands/solutions/3366213/rust-dfs/
impl Solution {

	pub fn foo(grid: &mut Vec<Vec<char>>, pos: (usize, usize)){
		let (r, c) = pos;
		if r < 0 || c < 0 || r > grid.len()-1 || c > grid[0].len()-1 || grid[r][c] != '1' {
			return;
		}

		grid[r][c] = '0';
		Self::foo(grid, (r-1,c));
		Self::foo(grid, (r+1,c));
		Self::foo(grid, (r,c+1));
		Self::foo(grid, (r,c-1));

	}


	pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
		let mut tmp = grid.clone();
		let mut res: i32 = 0;

		for i in 0..tmp.len() {
			for j in 0..tmp[0].len() {
				if ('1' == tmp[i][j]) {
					Self::foo(&mut tmp, (i,j));
					res += 1; 
				}
			}
		}

		return res;
	}
}