// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/solutions/3209451/rust-bfs-with-vecdeque/
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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        if let Some(n) = root {
            let mut queue = VecDeque::new();

            queue.push_back(n.clone());
            result.push(vec![n.clone().borrow().val]);
            let mut next_level = vec![];
            let mut direction = true;

            while let Some(node) = queue.pop_front() {
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();

                let level_finished = queue.is_empty();

                if left.is_some() {
                    next_level.push(left.clone().unwrap());
                }

                if right.is_some() {
                    next_level.push(right.clone().unwrap());
                }

                if level_finished {
                    for n in &next_level {
                        queue.push_back(n.clone());
                    }
                    if direction {
                        direction = false;
                    } else {
                        next_level.reverse();
                        direction = true;
                    }
                    let mut level = vec![];

                    for _n in 0..next_level.len() {
                        level.push(next_level.pop().clone().unwrap().borrow().val);
                    }
                    if !level.is_empty() {
                        result.push(level);
                    }
                }
            }
        }
        result
    }
}