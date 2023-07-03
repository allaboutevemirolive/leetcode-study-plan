// https://leetcode.com/problems/same-tree/solutions/343864/rust-0ms/
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // the better solution, one-liner: p == q
        fn f(p: Option<&Rc<RefCell<TreeNode>>>, q: Option<&Rc<RefCell<TreeNode>>>) -> bool {
            match (p, q) {
                (None, None) => true,
                (Some(p), Some(q)) => {
                    let p = p.borrow();
                    let q = q.borrow();
                    p.val == q.val
                        && f(p.left.as_ref(), q.left.as_ref())
                        && f(p.right.as_ref(), q.right.as_ref())
                }
                _ => false,
            }
        }
        f(p.as_ref(), q.as_ref())
    }
}