// https://leetcode.com/problems/maximum-depth-of-binary-tree/solutions/3180145/rust-recursive-and-iterative/
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

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {

    pub fn max_depth_recursive(root: Node) -> i32 {
        fn helper(root: &Node, current_depth: i32) -> i32 {
            match root {
                None => current_depth,
                Some(node) => {
                    let node = node.borrow();
                    let left_depth = helper(&node.left, current_depth + 1);
                    let right_depth = helper(&node.right, current_depth + 1);
                    std::cmp::max(left_depth, right_depth)
                }
            }
        }

        helper(&root, 0)
    }



    pub fn max_depth_non_recursive(root: Node) -> i32 {
        let mut stack = vec![];
        let mut max_depth = 0;
        if let Some(node) = root {
            stack.push((node, 1));
            while !stack.is_empty() {
                let (current_node, depth) = stack.pop().unwrap();
                max_depth = std::cmp::max(max_depth, depth);

                let current_node = current_node.borrow();
                if let Some(left) = &current_node.left {
                    stack.push((left.clone(), depth + 1));
                }
                if let Some(right) = &current_node.right {
                    stack.push((right.clone(), depth + 1));
                }
            }
        }
        max_depth
    }
    
    
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::max_depth_non_recursive(root)
    }
}