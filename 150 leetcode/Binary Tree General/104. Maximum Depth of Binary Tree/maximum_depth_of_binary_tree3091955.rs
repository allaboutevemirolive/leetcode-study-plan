// https://leetcode.com/problems/maximum-depth-of-binary-tree/solutions/3091955/rust-easy-recursion/
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                cmp::max(
                    Solution::max_depth(node.borrow().left.clone()),
                    Solution::max_depth(node.borrow().right.clone()),
                ) + 1
            }
            None => 0,
        }

    }
}