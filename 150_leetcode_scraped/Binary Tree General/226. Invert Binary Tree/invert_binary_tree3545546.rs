// https://leetcode.com/problems/invert-binary-tree/solutions/3545546/rust/
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
        if root == None {
            return None;
        }

        let mut roots = root.clone();
        let mut root_ref = roots.as_mut().unwrap().borrow_mut();

        let left = root_ref.left.take();
        let right = root_ref.right.take();

        root_ref.left = Solution::invert_tree(right);
        root_ref.right = Solution::invert_tree(left);

        root
    }
}