// https://leetcode.com/problems/symmetric-tree/solutions/3290506/rust-depth-first-solution/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(n1: &Option<Rc<RefCell<TreeNode>>>, n2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (n1, n2) {
                (None, None) => true,
                (Some(n1), Some(n2)) => {
                    let (n1, n2) = (n1.borrow(), n2.borrow());
                    n1.val == n2.val && dfs(&n1.left, &n2.right) && dfs(&n1.right, &n2.left)
                }
                _ => false,
            }
        }
        dfs(&root, &root)
    }
}