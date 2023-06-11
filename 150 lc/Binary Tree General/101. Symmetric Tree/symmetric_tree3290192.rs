// https://leetcode.com/problems/symmetric-tree/solutions/3290192/rust/
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(root) => {
                let mut root = root.borrow_mut();
                return Self::foo(root.left.take(), root.right.take());
            }
            None => true,
        }
    }

    fn foo(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(left), Some(right)) => {
                let mut left = left.borrow_mut();
                let mut right = right.borrow_mut();
                if left.val != right.val {
                    return false;
                }
                return Self::foo(left.left.take(), right.right.take()) &&
                    Self::foo(left.right.take(), right.left.take());
            }
            _ => false,
        }
    }
}