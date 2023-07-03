// https://leetcode.com/problems/validate-binary-search-tree/solutions/2729791/rust-clean-simple-recursive-dfs/
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
            match node {
                None => true,
                Some(node) => {
                    let node = node.borrow();
                    let val = node.val as i64;

                    if val <= min || val >= max { return false; }

                    dfs(node.left.clone(), min, val) && dfs(node.right.clone(), val, max)
                }
            }
        }

        dfs(root, i64::MIN, i64::MAX)
    }
}