// https://leetcode.com/problems/invert-binary-tree/solutions/3199138/rust-solution/
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref n) = node {
            let left = n.borrow().left.clone();
            let right = n.borrow().right.clone();

            let mut n = n.borrow_mut();
            n.left = Self::invert_tree(right);
            n.right = Self::invert_tree(left);
        }
        node
    }
}