// https://leetcode.com/problems/path-sum/solutions/3337433/rust-recursive/
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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        fn _has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32, sum: i32) -> bool {
            return match root {
                None => false,
                Some(cur) => {
                    let sum = sum + cur.borrow().val;
                    if cur.borrow().left.is_none() && cur.borrow().right.is_none() {
                        return sum == target_sum;
                    }
                    _has_path_sum(cur.borrow().left.clone(), target_sum, sum) || _has_path_sum(cur.borrow().right.clone(), target_sum, sum)
                }
            }
        }

        _has_path_sum(root, target_sum, 0)
    }
}