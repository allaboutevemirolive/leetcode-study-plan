// https://leetcode.com/problems/path-sum/solutions/3069455/rust-iterative-dfs/
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
use std::collections::VecDeque;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {

        let mut stack: Vec<(i32, Option<Rc<RefCell<TreeNode>>>)> = Vec::new();

        if let Some(v) = root {
            let val = v.borrow().val;
            stack.push((val, Some(v)));
        }
        while let Some((sum, Some(node))) = stack.pop() {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() && sum == target_sum {
                    return true;
            }

            if let Some(v) = node.left.clone() {
                let sum = v.borrow().val + sum;
                stack.push((sum, Some(v)));
            }
            if let Some(v) = node.right.clone() {
                let sum = v.borrow().val + sum;
                stack.push((sum, Some(v)));
            }
        }       
        false
    }
}