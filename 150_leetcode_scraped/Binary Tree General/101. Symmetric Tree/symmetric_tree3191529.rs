// https://leetcode.com/problems/symmetric-tree/solutions/3191529/rust-solution/
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
    pub fn solve(left : &Option<Rc<RefCell<TreeNode>>>,right : &Option<Rc<RefCell<TreeNode>>>) -> bool {
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

                ans &= Self::solve(&n.left,&b.right);
                ans &= Self::solve(&n.right,&b.left);
            }
        }

        return ans;
    }
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(b) = root {
            let n = b.borrow();
            return Self::solve(&n.left,&n.right);
        }
        return false
    }
}