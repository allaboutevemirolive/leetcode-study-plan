// https://leetcode.com/problems/maximum-depth-of-binary-tree/solutions/3192491/rust-solution-beats-100/
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
       pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match root {
                Some(node) => {
                    let borrowed = node.borrow();
                    let left_depth = helper(&borrowed.left);
                    let right_depth = helper(&borrowed.right);
                    1 + std::cmp::max(left_depth, right_depth)
                },
                None => 0
            }
        }
        
        helper(&root)
    }
}