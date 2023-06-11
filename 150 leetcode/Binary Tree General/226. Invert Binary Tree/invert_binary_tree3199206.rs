// https://leetcode.com/problems/invert-binary-tree/solutions/3199206/recursive-rust/
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(mut node) =  root.as_ref().map(|n| n.borrow_mut()) {
            Self::invert_tree(node.left.as_ref().map(|n| Rc::clone(n)));
            Self::invert_tree(node.right.as_ref().map(|n| Rc::clone(n)));
            let r = node.left.take();
            node.left = node.right.take();
            node.right = r;
        }
        root
    }
}