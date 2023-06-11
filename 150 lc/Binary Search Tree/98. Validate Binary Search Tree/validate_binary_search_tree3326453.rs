// https://leetcode.com/problems/validate-binary-search-tree/solutions/3326453/rust-in-order-traversal-solution-o-n-runtime-and-space/
use std::{cell::RefCell, rc::Rc};

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    fn helper(node: Option<Node>, previous: &mut Option<i32>, is_valid: &mut bool) {
        if let Some(node) = node {
            Self::helper(node.borrow().left.clone(), previous, is_valid);
            if let Some(previous) = previous.as_mut() {
                if node.borrow().val <= *previous {
                    *is_valid = false;
                    return;
                }
            }
            previous.replace(node.borrow().val);
            Self::helper(node.borrow().right.clone(), previous, is_valid);
        }
    }

    pub fn is_valid_bst(root: Option<Node>) -> bool {
        let (mut previous, mut is_valid) = (None, true);
        Self::helper(root, &mut previous, &mut is_valid);
        is_valid
    }
}