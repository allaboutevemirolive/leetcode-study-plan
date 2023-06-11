// https://leetcode.com/problems/maximum-depth-of-binary-tree/solutions/3366208/rust-dfs/
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

impl Solution {

	pub fn foo(node: Option<Rc<RefCell<TreeNode>>>, lvl: i32, res: &mut i32) {
		match node {
			Some(nd) => {
				*res = max(*res, lvl);
				Self::foo(nd.borrow().left.clone(), lvl+1, res);
				Self::foo(nd.borrow().right.clone(), lvl+1, res);
			},
			None => return,
		};
	}

	pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let mut res: i32 = 0;
		Self::foo(root, 1, &mut res);
		res
	}
}