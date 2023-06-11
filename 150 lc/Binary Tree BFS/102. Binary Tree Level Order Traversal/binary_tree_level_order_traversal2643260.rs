// https://leetcode.com/problems/binary-tree-level-order-traversal/solutions/2643260/rust-0ms-recursive/
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
type Node = Rc<RefCell<TreeNode>>;

impl Solution {

    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if let Some(r) = root {
            let mut ret = vec![vec![r.borrow().val]];
            let mut next_nodes = vec![];
            if let Some(left) = r.borrow().left.as_ref() {
                next_nodes.push(left.clone());
            }
            if let Some(right) = r.borrow().right.as_ref() {
                next_nodes.push(right.clone());
            }
            Self::next_level(next_nodes, &mut ret);
            ret
        } else {
            vec![]
        }
    }

    fn next_level(nodes: Vec<Node>, ret: &mut Vec<Vec<i32>>) {
        if nodes.is_empty() {
            return;
        }
        let mut values = vec![];
        let mut next_nodes = vec![];
        for node in nodes {
            values.push(node.borrow().val);
            if let Some(left) = node.borrow().left.as_ref() {
                next_nodes.push(left.clone());
            }
            if let Some(right) = node.borrow().right.as_ref() {
                next_nodes.push(right.clone());
            }
        }
        if !values.is_empty() {
            ret.push(values);
        }
        Self::next_level(next_nodes, ret);
    }

}