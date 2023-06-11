// https://leetcode.com/problems/kth-smallest-element-in-a-bst/solutions/3168332/rust-iterative-inorder-traversal-with-stack/
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

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn kth_smallest(mut root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        let mut stack = vec![];
        while root.is_some() || !stack.is_empty() {
            while let Some(node) = root {
                root = node.borrow_mut().left.take();
                stack.push(node);
            }
            if let Some(node) = stack.pop() {
                k -= 1;
                if k == 0 {
                    return node.borrow().val;
                }
                root = node.borrow_mut().right.take();
            }
        }
        -1
    }
}