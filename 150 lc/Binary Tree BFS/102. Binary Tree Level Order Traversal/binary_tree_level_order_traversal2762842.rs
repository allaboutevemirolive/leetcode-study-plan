// https://leetcode.com/problems/binary-tree-level-order-traversal/solutions/2762842/rust-bfs-0ms/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut q = VecDeque::new();
        if let Some(node) = root {
            q.push_back(node);
        } else {
            return Vec::new();
        }

        let mut result = Vec::new();
        while !q.is_empty() {
            let mut curr_level = Vec::new();

            let mut level_cnt = q.len();
            while let Some(node) = q.pop_front() {
                let mut node = node.borrow_mut();
                if let Some(left) = node.left.take() {
                    q.push_back(left);
                }
                if let Some(right) = node.right.take() {
                    q.push_back(right);
                }
                curr_level.push(node.val);

                level_cnt -= 1;
                if level_cnt == 0 {
                    break;
                }
            }
            result.push(curr_level);
        }
        result
    }
}