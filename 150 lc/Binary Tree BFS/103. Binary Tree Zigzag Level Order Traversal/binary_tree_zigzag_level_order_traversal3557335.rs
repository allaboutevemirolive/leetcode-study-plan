// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/solutions/3557335/rust/
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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut v = vec![];
        let mut queue = std::collections::VecDeque::new();
        if root.is_none(){
            return vec![];
        }
        queue.push_back((0,root.clone()));
        while queue.len() > 0 {
            let bound = queue.len() - 1;
            for i in 0..=bound {
                let (level,node) = queue.pop_front().unwrap();
                v.push(node.clone().unwrap().borrow().val);
                    if i == bound {
                        res.push((level,v.clone()));
                        v = vec![];
                    }
                    if let Some(l) = node.clone().unwrap().borrow().left.clone() {
                        queue.push_back((level + 1 ,Some(l)));
                    }
                    if let Some(r) = node.clone().unwrap().borrow().right.clone() {
                        queue.push_back((level + 1 ,Some(r)));
                    }
            }
        }
        let mut ans:Vec<Vec<i32>> = vec![];
        for (level,val) in res.into_iter()  {
            if level % 2 == 1 {
                ans.push(val.into_iter().rev().collect::<Vec<i32>>());
            }else{
                ans.push(val);
            }
        }
        ans
    }
}