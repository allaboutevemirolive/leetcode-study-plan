// https://leetcode.com/problems/maximum-depth-of-binary-tree/solutions/3193146/rust-clean-dfs-bfs-with-move-and-borrowing/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node_rc) => {
                let mut node = node_rc.borrow_mut();
                1 + Self::max_depth(node.left.take()).max(Self::max_depth(node.right.take()))
            },
            None => 0,
        }
    }
}