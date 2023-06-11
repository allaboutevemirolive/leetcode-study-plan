// https://leetcode.com/problems/sqrtx/solutions/2958809/rust-binary-search/
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
    let mut l:u64 = 0;
	let mut m:u64 = 0;
	let mut r:u64 = x as u64 + 1;

    while l != r - 1{
        m = (l + r) / 2;

		if m * m <= x as u64{
			l = m;
        }
		else{
			r = m;
        }
	}

    l as i32
    }
}