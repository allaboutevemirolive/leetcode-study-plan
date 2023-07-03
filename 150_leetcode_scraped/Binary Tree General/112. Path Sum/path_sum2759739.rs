// https://leetcode.com/problems/path-sum/solutions/2759739/simple-rust-code/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(node) => {
                let node = node.borrow();
                let rest = target_sum - node.val;
                if Self::is_leaf(&node) {
                    rest == 0
                } else {
                    Self::has_path_sum(node.left.clone(), rest) || Self::has_path_sum(node.right.clone(), rest)
                }
            }

            None => false,
        }
    }

    fn is_leaf(node: &TreeNode) -> bool {
        match node {
            TreeNode { val: _, left: None, right: None } => true,
            _ => false
        }
    }
}