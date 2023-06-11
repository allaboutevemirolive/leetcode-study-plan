// https://leetcode.com/problems/path-sum/solutions/2660220/rust-0ms-100-faster/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(ref_node) = root {
            let mut node = ref_node.borrow_mut();
            let target_sum = target_sum - node.val;
            if node.left.is_none() && node.right.is_none() {
                target_sum == 0
            } else {
                Self::has_path_sum(node.left.take(), target_sum)
                    || Self::has_path_sum(node.right.take(), target_sum)
            }
        } else {
            false
        }
    }
}