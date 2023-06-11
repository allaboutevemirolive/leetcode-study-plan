// https://leetcode.com/problems/same-tree/solutions/657006/rust-recursion-0ms/
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(p), Some(q)) => {
            return p.borrow().val == q.borrow().val
                && is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                && is_same_tree(p.borrow().right.clone(), q.borrow().right.clone())
        }
        (None, None) => true,
        _ => false,
    }
}