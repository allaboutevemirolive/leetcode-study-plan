// https://leetcode.com/problems/kth-smallest-element-in-a-bst/solutions/3216755/rust-inorder-iterative-solution/
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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
let mut stack = vec![];
    let mut res_counter = 0;
    let mut tree = root;
    while !stack.is_empty() || tree.is_some() {
        match  tree {
            Some(node) => {
                stack.push(node.clone());
                // tree = node.borrow().left.take();
                tree = node.borrow_mut().left.take();
                
            },
            None => {
                let node = stack.pop().unwrap();
                res_counter +=1;
                if res_counter == k as usize {
                    return node.borrow().val
                }
                let right  = node.borrow_mut().right.take();
                tree = right;
            }
        }
    }
    -1            
    }
}