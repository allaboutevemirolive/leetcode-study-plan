// https://leetcode.com/problems/minimum-absolute-difference-in-bst/solutions/223753/rust-solution/
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
use std::i32::{MIN, MAX};
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Self::traverse(root, MIN, MAX).1;
    }
    
    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, last: i32, res: i32) -> (i32, i32) {
        if let Some(r) = root {
            let (mut last, mut res) = Self::traverse(r.borrow().left.clone(), last, res);
            res = res.min(r.borrow().val.saturating_sub(last));
            last = r.borrow().val;
            return Self::traverse(r.borrow().right.clone(), last, res);
        }
        return (last, res);
    }
}