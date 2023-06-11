// https://leetcode.com/problems/maximum-depth-of-binary-tree/solutions/3198263/rust-solution/
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
    pub fn solve(root: &Option<Rc<RefCell<TreeNode>>>,ans : &mut i32,res : i32) {
        if let Some(v) = root {
            let n = v.borrow();

            Self::solve(&n.left,ans,res + 1);
            Self::solve(&n.right,ans,res + 1);
        }
        else {
            if *ans < res {
                *ans = res;
            }
        }

    }
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans : i32 = 0;
        Self::solve(&root,&mut ans,0);
        ans
    }
}