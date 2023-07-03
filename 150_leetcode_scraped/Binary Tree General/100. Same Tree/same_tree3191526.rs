// https://leetcode.com/problems/same-tree/solutions/3191526/rust-solution/
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
    pub fn solve(left: &Option<Rc<RefCell<TreeNode>>>, right: &Option<Rc<RefCell<TreeNode>>>) -> bool{
         if left.is_none() && right.is_none(){
            return true;
        }

        if left.is_none() {
            return false;
        }

        if right.is_none() {
            return false;
        }

        let mut ans = true;

        if let Some(v) = left {
            if let Some(v1) = right {
                let n = v.borrow();
                let b = v1.borrow();

                if n.val != b.val {
                    return false;
                }

                ans &= Self::solve(&n.left,&b.left);
                ans &= Self::solve(&n.right,&b.right);
            }
        }

        return ans;
    }
    pub fn is_same_tree(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::solve(&left,&right)
    }
}