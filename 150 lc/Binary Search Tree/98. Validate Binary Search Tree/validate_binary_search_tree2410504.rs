// https://leetcode.com/problems/validate-binary-search-tree/solutions/2410504/rust-recursive-simple/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_tree_in_range(root, i64::MIN, i64::MAX)
    }

    pub fn is_tree_in_range(root: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            let val = node.val as i64;
            if val > min && val < max {
                Self::is_tree_in_range(node.left.clone(), min, val) &&
                Self::is_tree_in_range(node.right.clone(), val, max)
            } else {
                false
            }
        } else {
            true
        }
    }
}