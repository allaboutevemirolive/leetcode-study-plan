// https://leetcode.com/problems/path-sum/solutions/3554015/rust/
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
        let mut res: Vec<Vec<i32>> = vec![];
        let mut stack: Vec<(Vec<i32>, Option<Rc<RefCell<TreeNode>>>)> = Vec::new();
        if root.is_none(){
            return false;
        }
        stack.push((vec![root.clone().unwrap().borrow().val], root.clone()));
        while let Some((vec, Some(node))) = stack.pop() {
            let n = node.borrow();
            if n.left.is_none() && n.right.is_none() {
                res.push(vec.clone());
            }
            if let Some(l) = n.left.clone() {
                let mut v = vec.clone();
                v.push(l.borrow().val);
                stack.push((v, Some(l)));
            }
            if let Some(r) = n.right.clone() {
                let mut v = vec.clone();
                v.push(r.borrow().val);
                stack.push((v, Some(r)));
            }
        }
        for r in res.into_iter(){
            if r.into_iter().sum::<i32>() == target_sum {
                return true;
            }
        }
        false
    }
}