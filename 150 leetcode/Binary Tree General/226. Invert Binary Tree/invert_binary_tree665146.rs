// https://leetcode.com/problems/invert-binary-tree/solutions/665146/python-rust-depth-first-search/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let right = node.borrow().right.clone();
            let left = node.borrow().left.clone();
            node.borrow_mut().left = Self::invert_tree(right);
            node.borrow_mut().right = Self::invert_tree(left);
            return Some(node);
        } else {
            return None;
        }
    }
}