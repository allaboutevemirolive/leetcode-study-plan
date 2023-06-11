// https://leetcode.com/problems/path-sum/solutions/3313692/rust-pattern-matching/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {    
        match root {
            Some(root) => {
                let root = root.borrow();
                if root.left.is_none() && root.right.is_none() {
                    root.val == target_sum
                } else {
                    Solution::has_path_sum(root.left.clone(), target_sum - root.val) || Solution::has_path_sum(root.right.clone(), target_sum - root.val)
                }        
            }
            None => false,
        }
    }
}