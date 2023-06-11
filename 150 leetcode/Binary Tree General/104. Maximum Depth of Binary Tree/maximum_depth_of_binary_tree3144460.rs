// https://leetcode.com/problems/maximum-depth-of-binary-tree/solutions/3144460/rust-dfs-recursive-iterative-100-solutions/
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                1 + max(Self::max_depth(node.borrow().left.clone()), Self::max_depth(node.borrow().right.clone()))
            }
        }
    }
}