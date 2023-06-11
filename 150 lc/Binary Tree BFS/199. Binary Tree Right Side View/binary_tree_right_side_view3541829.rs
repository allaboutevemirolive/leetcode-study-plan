// https://leetcode.com/problems/binary-tree-right-side-view/solutions/3541829/rust-o-n-solution-using-bfs-breadth-first-search/
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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // Check if the root is None
        if root.is_none() {
            return vec![];
        }
        let mut result = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());

        while !queue.is_empty() {
            let queue_size = queue.len();

            for index in 0..queue_size {
                let node = queue.pop_front().unwrap();
                let left = &node.borrow().left;
                let right = &node.borrow().right;

                // Push the left child into the queue if it exists
                if left.is_some() {
                    queue.push_back(Rc::clone(left.as_ref().unwrap()));
                }

                // Push the right child into the queue if it exists
                if right.is_some() {
                    queue.push_back(Rc::clone(right.as_ref().unwrap()));
                }

                // If it is the rightmost node at this level, add its value to the result
                if index == queue_size - 1 {
                    result.push(node.borrow().val);
                }
            }
        }
        result
    }
}
