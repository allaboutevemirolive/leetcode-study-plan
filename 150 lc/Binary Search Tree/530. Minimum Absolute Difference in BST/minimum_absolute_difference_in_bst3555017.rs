// https://leetcode.com/problems/minimum-absolute-difference-in-bst/solutions/3555017/rust/
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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res: Vec<i32> = vec![];
        let mut stack: Vec<(i32, Option<Rc<RefCell<TreeNode>>>)> = Vec::new();
        if root.is_none(){
            return 0;
        }
        stack.push((root.clone().unwrap().borrow().val, root.clone()));
        while let Some((val, Some(node))) = stack.pop() {
            let n = node.borrow();
            res.push(val);
            if let Some(l) = n.left.clone() {
                stack.push((l.borrow().val, Some(l.clone())));
            }
            if let Some(r) = n.right.clone() {
                stack.push((r.borrow().val, Some(r.clone())));
            }
        }
        res.sort();
        let n = res.windows(2);
        let ans = n.into_iter().map(|m| i32::abs(m[0] - m[1])).collect::<Vec<i32>>();
        ans.into_iter().min().unwrap()
    }
}