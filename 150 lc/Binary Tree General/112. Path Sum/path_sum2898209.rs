// https://leetcode.com/problems/path-sum/solutions/2898209/rust-dfs-recursion-beats-100-performance-97-memory/
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
type OpRcNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::dfs(&root, target_sum)
    }

    fn dfs(node: &OpRcNode, target: i32) -> bool {
        match node {
            Some(node) => {
                let n = node.borrow();
                match (&n.left, &n.right) {
                    (None, None) => n.val == target,
                    _ => Self::dfs(&n.left, target - n.val)
                        || Self::dfs(&n.right, target - n.val)
                }
            },
            None => false,
        }
    }
}