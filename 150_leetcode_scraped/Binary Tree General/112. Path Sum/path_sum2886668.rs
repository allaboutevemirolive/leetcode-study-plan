// https://leetcode.com/problems/path-sum/solutions/2886668/rust-0ms-simple-dfs/
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
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
            if root.is_none() {
                return false;
            }

            let root = root.clone().unwrap();
            let root = root.borrow();

            let t_sum = target_sum - root.val;

            if root.left.is_none() && root.right.is_none() {
                return t_sum == 0;
            }

            dfs(root.left.clone(), t_sum) || dfs(root.right.clone(), t_sum)
        }

        dfs(root, target_sum)
    }
}