// https://leetcode.com/problems/kth-smallest-element-in-a-bst/solutions/3253903/rust-recursive-solution-beats-100/
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
use std::{rc::Rc, cell::RefCell};

impl Solution {
    fn _kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
        if let Some(node) = root {
            Self::_kth_smallest(node.borrow().left.clone(), nums);
            nums.push(node.borrow().val);
            Self::_kth_smallest(node.borrow().right.clone(), nums);
        }
    }

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut nums = Vec::new();
        Self::_kth_smallest(root, &mut nums);
        nums[(k - 1) as usize]
    }
}
