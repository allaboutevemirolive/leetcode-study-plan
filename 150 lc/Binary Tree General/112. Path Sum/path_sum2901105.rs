// https://leetcode.com/problems/path-sum/solutions/2901105/simple-rust-pattern-matching/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        root.map_or(false, |root| {
            match &*root.borrow() {
                &TreeNode {val, left: None, right: None} => val == target_sum,
                &TreeNode {val, ref left, ref right} => 
                    Self::has_path_sum(left.clone(), target_sum - val) || Self::has_path_sum(right.clone(), target_sum - val),
            }
        })
    }
}