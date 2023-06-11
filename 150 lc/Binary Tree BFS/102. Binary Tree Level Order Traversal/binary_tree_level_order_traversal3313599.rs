// https://leetcode.com/problems/binary-tree-level-order-traversal/solutions/3313599/rust-recursive-solution/
use {
    std::{cell::RefCell, rc::Rc},
};

impl Solution {
    fn _level_order(node: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<Vec<i32>>, level: usize) {
        if let Some(node) = node {
            if let Some(values) = values.get_mut(level) {
                values.push(node.borrow().val);
            } else {
                values.push(vec![node.borrow().val]);
            }
            Self::_level_order(node.borrow().left.clone(), values, level + 1);
            Self::_level_order(node.borrow().right.clone(), values, level + 1);
        }
    }

    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut values = Vec::new();
        Self::_level_order(root, &mut values, 0);
        values
    }
}
