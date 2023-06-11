// https://leetcode.com/problems/path-sum/solutions/2659080/rust-0ms-recursive/
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            None => false,
            Some(node) => match (node.borrow().left.as_ref(), node.borrow().right.as_ref()) {
                (None, None) => target_sum - node.borrow().val == 0,
                (_, _) => {
                    let new_target = target_sum - node.borrow().val;
                    Solution::has_path_sum(node.borrow().left.clone(), new_target)
                        || Solution::has_path_sum(node.borrow().right.clone(), new_target)
                }
            },
        }
    }
}