// https://leetcode.com/problems/same-tree/solutions/3336790/rust-implementation/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(p_ref), Some(q_ref)) => {
                if p_ref.borrow().val != q_ref.borrow().val {
                    false
                } else {
                    Self::is_same_tree(p_ref.borrow().left.clone(), q_ref.borrow().left.clone())
                        && Self::is_same_tree(
                            p_ref.borrow().right.clone(),
                            q_ref.borrow().right.clone(),
                        )
                }
            }
        }
    }
}
