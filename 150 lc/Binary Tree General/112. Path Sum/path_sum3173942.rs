// https://leetcode.com/problems/path-sum/solutions/3173942/rust-simple-solution/
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
    pub fn solve(root: &Option<Rc<RefCell<TreeNode>>>,target : i32) -> bool{

        let mut ans = false;
        if let Some(v) = root {
            let n = v.borrow();

            if n.left.is_none() && n.right.is_none() {
                return n.val == target;
            }
            ans |= Self::solve(&n.left,target-n.val);
            ans |= Self::solve(&n.right,target-n.val);
        }

        ans
    }
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        return Self::solve(&root,target_sum);
    }
}