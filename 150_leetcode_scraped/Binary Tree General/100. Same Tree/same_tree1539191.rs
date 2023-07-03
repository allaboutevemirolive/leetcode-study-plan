// https://leetcode.com/problems/same-tree/solutions/1539191/rust-recursive-approach/
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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::is_same_tree_recursive_impl(&p, &q)
    }

    pub fn is_same_tree_recursive_impl(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        } else if p.is_none() && !q.is_none() {
            return false;
        } else if !p.is_none() && q.is_none() {
            return false;
        }

        let a: &TreeNode = &p.as_ref().unwrap().borrow();
        let b: &TreeNode = &q.as_ref().unwrap().borrow();

        if a.val != b.val {
            return false;
        }

        Solution::is_same_tree_recursive_impl(&a.left, &b.left) &&
            Solution::is_same_tree_recursive_impl(&a.right, &b.right)
    }
}