// https://leetcode.com/problems/invert-binary-tree/solutions/3103259/short-and-simple-rust-solution/
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::DerefMut;

pub fn traverse(node_maybe: &mut Option<Rc<RefCell<TreeNode>>>) {
    let node = if let Some(node) = node_maybe.as_mut() { node } else { return };
    let mut node_inner = node.borrow_mut();
    let node_deref = node_inner.deref_mut();
    std::mem::swap(&mut node_deref.left, &mut node_deref.right);
    traverse(&mut node_deref.left);
    traverse(&mut node_deref.right);
}

impl Solution {
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        traverse(&mut root);
        root
    }
}