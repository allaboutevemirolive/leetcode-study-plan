// https://leetcode.com/problems/maximum-depth-of-binary-tree/solutions/3192358/rust-optimized-recursion-0-ms/
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;

impl Solution {

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => inner_max_depth(&node),
        }
    }    
}

fn inner_max_depth(root: &Rc<RefCell<TreeNode>>) -> i32 {
    match (root.borrow().left.clone(), root.borrow().right.clone()) {
        (None, None) => 1,
        (Some(l), None) => 1 + inner_max_depth(&l),
        (None, Some(r)) => 1 + inner_max_depth(&r),
        (Some(l), Some(r)) => 1 + cmp::max(inner_max_depth(&l), inner_max_depth(&r)),
    }
}