// https://leetcode.com/problems/maximum-depth-of-binary-tree/solutions/3245129/rust-recursive-dfs/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            1 + Self::max_depth(node.borrow().left.clone()).max(Self::max_depth(node.borrow().right.clone()))
        } else {
            0
        }
    }
}