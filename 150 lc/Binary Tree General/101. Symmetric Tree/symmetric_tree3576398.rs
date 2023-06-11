// https://leetcode.com/problems/symmetric-tree/solutions/3576398/rust-simply-solution/
	use std::rc::Rc;
	use std::cell::RefCell;
	impl Solution {
		pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
			fn foo(nd1: Option<Rc<RefCell<TreeNode>>>, nd2: Option<Rc<RefCell<TreeNode>>>) -> bool {
				match (nd1, nd2) {
					(None, None) => true,
					(Some(nd1), Some(nd2)) => {
						nd1.borrow().val == nd2.borrow().val && foo(nd1.borrow().left.clone(), nd2.borrow().right.clone()) &&
						foo(nd1.borrow().right.clone(), nd2.borrow().left.clone())
					}
					_ => false,
				}
			}
			if let Some(node) = root {
				return foo(node.borrow().left.clone(), node.borrow().right.clone());
			}
			true
		}
	}