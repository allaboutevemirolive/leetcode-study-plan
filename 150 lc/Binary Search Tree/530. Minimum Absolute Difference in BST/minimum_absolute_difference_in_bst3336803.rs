// https://leetcode.com/problems/minimum-absolute-difference-in-bst/solutions/3336803/rust-implementation/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, order: &mut Vec<i32>) {
            if let Some(root_ref) = root {
                let root_node = root_ref.borrow();
                dfs(&root_node.left, order);
                order.push(root_node.val);
                dfs(&root_node.right, order);
            }
        }
        let mut order = Vec::new();
        dfs(&root, &mut order);
        let mut minimum_diff = i32::MAX;
        for index in 1..order.len() {
            minimum_diff = minimum_diff.min(order[index] - order[index - 1]);
        }
        minimum_diff
    }
}