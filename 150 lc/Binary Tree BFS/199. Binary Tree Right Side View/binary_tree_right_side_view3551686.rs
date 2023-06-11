// https://leetcode.com/problems/binary-tree-right-side-view/solutions/3551686/rust/
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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        while queue.len() > 0 {
            let bound = queue.len() - 1;
            for i in 0..=bound{
                let node = queue.pop_front().unwrap();
                if i == bound{
                    if let Some(n) = node.clone(){
                        res.push(n.borrow().val);
                    }
                }
                if let Some(n) = node.clone(){
                    if let Some(l) = n.clone().borrow().left.clone() {
                        queue.push_back(Some(l.clone()));
                    }
                }
                if let Some(n) = node.clone(){
                    if let Some(r) = n.clone().borrow().right.clone(){
                        queue.push_back(Some(r.clone()));
                    }
                }
            }
        }
        res
    }
}