// https://leetcode.com/problems/path-sum/solutions/2844689/rust-match-statement-with-struct-patterns/
use std::rc::Rc;
use std::cell::RefCell;

type NodeOpt = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn has_path_sum(root: NodeOpt, mut target_sum: i32) -> bool {
        root.map_or(false, |node| {
            match &*node.borrow() {
                TreeNode { val, left: None, right: None } => {
                    target_sum - val == 0
                },
                TreeNode { val, left, right } => {
                    Self::has_path_sum(left.clone(), target_sum - val) ||
                    Self::has_path_sum(right.clone(), target_sum - val)
                }
            }
        })
    }
}