// https://leetcode.com/problems/symmetric-tree/solutions/3292775/rust-recursive-solution/
use std::cell::RefCell;
use std::collections::VecDeque;
use std::mem::swap;
use std::rc::Rc;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(tree) => Self::is_mirror(tree.borrow().left.clone(), tree.borrow().right.clone()),
            None => true,
        }
    }

    fn is_mirror(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(l), Some(r)) => {
                let l = l.borrow();
                let r = r.borrow();

                if l.val != r.val {
                    false
                } else {
                    Self::is_mirror(l.left.clone(), r.right.clone())
                        && Self::is_mirror(l.right.clone(), r.left.clone())
                }
            }
        }
    }
}