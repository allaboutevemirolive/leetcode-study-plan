// https://leetcode.com/problems/same-tree/solutions/443886/rust/
impl Solution {
    pub fn is_same_tree(mut p: Option<Rc<RefCell<TreeNode>>>, mut q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        p.is_none() && q.is_none() ||
            p.is_some() && q.is_some() && p.as_ref().unwrap().borrow().val == q.as_ref().unwrap().borrow().val &&
            Self::is_same_tree(p.as_mut().unwrap().borrow_mut().left.take(), q.as_mut().unwrap().borrow_mut().left.take()) &&
            Self::is_same_tree(p.as_mut().unwrap().borrow_mut().right.take(), q.as_mut().unwrap().borrow_mut().right.take())
    }
}