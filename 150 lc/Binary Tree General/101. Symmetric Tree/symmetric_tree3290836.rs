// https://leetcode.com/problems/symmetric-tree/solutions/3290836/rust/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn is_mirror(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (Some(l), Some(r)) =>
                l.borrow().val == r.borrow().val &&
                Solution::is_mirror(l.borrow().left.clone(), r.borrow().right.clone()) &&
                Solution::is_mirror(l.borrow().right.clone(), r.borrow().left.clone()),
            (None, None) => true,
            _ => false,
        }
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root_node) = root {
            Solution::is_mirror(root_node.borrow().left.clone(), root_node.borrow().right.clone())
        } else {
            false
        }
    }
}