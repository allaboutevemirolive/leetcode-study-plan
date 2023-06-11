// https://leetcode.com/problems/average-of-levels-in-binary-tree/solutions/2517276/rust-and-javascript-solutions/
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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut avgs = vec![];
        let mut nodes = VecDeque::new();

        if let Some(node) = root {
            nodes.push_back(node);
        }

        while !nodes.is_empty() {
            let N = nodes.len();
            let mut sum = 0.0;

            for _ in 0..N {
                if let Some(node) = nodes.pop_front() {
                    sum += f64::from(node.borrow().val);

                    if let Some(left) = node.borrow_mut().left.take() {
                        nodes.push_back(left);
                    }

                    if let Some(right) = node.borrow_mut().right.take() {
                        nodes.push_back(right);
                    }
                }
            }

            avgs.push(sum / N as f64);
        }

        avgs
    }
}