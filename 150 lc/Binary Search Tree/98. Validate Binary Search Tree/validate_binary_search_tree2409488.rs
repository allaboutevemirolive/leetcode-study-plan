// https://leetcode.com/problems/validate-binary-search-tree/solutions/2409488/rust-simple-bfs/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::*;
 
impl Solution {
	pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
		let mut q = VecDeque::new();
		if let Some(p) = root { q.push_back((Rc::clone(&p), i64::MIN, i64::MAX)); }
		while let Some((node, mn, mx)) = q.pop_front() {
			if (node.borrow().val as i64) <= mn || (node.borrow().val as i64) >= mx {
				return false;
			}
			if let Some(l) = &node.borrow().left {
				q.push_back((Rc::clone(&l), mn, node.borrow().val as i64));
			}
			if let Some(r) = &node.borrow().right {
				q.push_back((Rc::clone(&r), node.borrow().val as i64, mx));
			}
		}
		true
	}
}