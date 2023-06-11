// https://leetcode.com/problems/symmetric-tree/solutions/3290677/rust-0ms-recursive-easy-to-follow-match-statement/
use std::cell::RefCell;
use std::rc::Rc;

type BTreeNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn compare_mirror(t1: &BTreeNode, t2: &BTreeNode) -> bool {
        match (t1, t2) {
            (None, None) => true,
            (None, _) | (_, None) => false,
            (Some(n1), Some(n2)) if n1.borrow().val != n2.borrow().val => {
                false
            }
            #[rustfmt::skip]
            (Some(n1), Some(n2)) => {
                Self::compare_mirror(&n1.borrow().left, &n2.borrow().right)
                &&
                Self::compare_mirror(&n1.borrow().right, &n2.borrow().left)
            },
        }
    }

    pub fn is_symmetric(root: BTreeNode) -> bool {
        match root {
            Some(root) => {
                Self::compare_mirror(&root.borrow().left, &root.borrow().right)
            }
            None => true,
        }
    }
}