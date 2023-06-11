// https://leetcode.com/problems/invert-binary-tree/solutions/2095722/rust-simple-recursive-iterative/
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// Given the root of a binary tree,
    /// invert the tree, and return its root.
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) {
            match node {
                Some(n) => {
                    dfs(&n.borrow().left);
                    dfs(&n.borrow().right);
                    let tree = &mut *n.borrow_mut();
                    std::mem::swap(&mut tree.left, &mut tree.right);
                }
                None => (),
            }
        }

        dfs(&root);
        root
    }
}