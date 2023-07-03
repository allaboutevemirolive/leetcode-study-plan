// https://leetcode.com/problems/invert-binary-tree/solutions/2961620/c-vs-rust/
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        invert_tree_inner(root)
    }
}

fn invert_tree_inner(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(ref node) = root {
        let mut _node = node.borrow_mut();
        let left_node = match _node.left {
            Some(ref subnode) => invert_tree_inner(Some(Rc::clone(subnode))),
            None => None,
        };
        let right_node = match _node.right {
            Some(ref subnode) => invert_tree_inner(Some(Rc::clone(subnode))),
            None => None,
        };
        _node.left = right_node;
        _node.right = left_node;
    }
    root
}