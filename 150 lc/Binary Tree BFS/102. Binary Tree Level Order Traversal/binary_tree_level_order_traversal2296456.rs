// https://leetcode.com/problems/binary-tree-level-order-traversal/solutions/2296456/rust-recursive-dfs-iterative-bfs/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
	pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
		fn traversal(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<Vec<i32>>, level: usize){
			if let Some(r) = root {
				if ans.len() == level {ans.push(Vec::new());}
				ans[level].push(r.borrow().val);
				traversal(r.borrow().left.clone(),ans,level+1);             
				traversal(r.borrow().right.clone(),ans,level+1);
			}
		}
		let mut ans: Vec<Vec<i32>> = Vec::new();
		traversal(root.clone(),&mut ans, 0);
		ans
	}
}