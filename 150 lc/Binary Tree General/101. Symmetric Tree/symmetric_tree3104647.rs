// https://leetcode.com/problems/symmetric-tree/solutions/3104647/rust-iterative-solution/
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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = Vec::new();
        let rt_node = root.unwrap();
        stack.push((rt_node.borrow().left.clone(), rt_node.borrow().right.clone()));

        while let Some(node_pair) = stack.pop() {
            let is_sym = match node_pair {
                (Some(l_node), Some(r_node)) => {
                    if l_node.borrow().val != r_node.borrow().val {
                        return false
                    }
                    stack.push((l_node.borrow().left.clone(), r_node.borrow().right.clone()));
                    stack.push((l_node.borrow().right.clone(), r_node.borrow().left.clone()));
                    true
                }
                (None, None) => true,
                _ => false
            };

            if !is_sym {
                return false;
            }
        }

        true
    }
}