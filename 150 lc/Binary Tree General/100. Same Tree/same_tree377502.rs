// https://leetcode.com/problems/same-tree/solutions/377502/rust-solution-recursive/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::eq(&p, &q)
    }
    fn eq(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (a, b) {
            (Some(ax), Some(bx)) if ax.borrow().val == bx.borrow().val => Solution::eq(&ax.borrow().left, &bx.borrow().left) && Solution::eq(&ax.borrow().right, &bx.borrow().right),
            (None, None) => true,
            _ => false
        }
    }
}