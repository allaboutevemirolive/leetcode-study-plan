// https://leetcode.com/problems/path-sum/solutions/2663462/rust-solution-0-ms/
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        fn inner(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
            if let Some(node) = root {
                let node = node.borrow();
                let left = node.left.clone();
                let right = node.right.clone();
                
                if node.val == target_sum && left.is_none() && right.is_none() {
                    return true;
                }
                
                inner(left, target_sum - node.val) || inner(right, target_sum - node.val)
            } else {
                false
            }
        }
        
        inner(root, target_sum)
    }
}