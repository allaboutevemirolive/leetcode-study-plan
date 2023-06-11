// https://leetcode.com/problems/climbing-stairs/solutions/3366163/rust-memo/
use std::collections::HashMap;

impl Solution {    
	pub fn climb_stairs(n: i32) -> i32 {        
		let mut da = vec![std::i32::MIN; (n+1) as usize];

		fn foo(n: i32, da: &mut Vec<i32>) -> i32{
			if n < 0{ return 0; }
			if n < 1 { return 1;}
			let nu = n as usize;
			if da[nu] == std::i32::MIN {
				da[nu] = foo(n-1, da) + foo(n-2, da);
			}
			return da[nu];
		}

		return foo(n, &mut da);
	}
}