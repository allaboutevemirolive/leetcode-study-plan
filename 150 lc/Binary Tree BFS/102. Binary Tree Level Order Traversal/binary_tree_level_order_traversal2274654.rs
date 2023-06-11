// https://leetcode.com/problems/binary-tree-level-order-traversal/solutions/2274654/rust-breadth-first-search-iterative-0ms/
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut que = Vec::new();
        let mut ret = Vec::new();
        if let Some(root) = root {
            que.push(root);
        }

        while !que.is_empty() {
            let mut level = Vec::with_capacity(que.len());
            let mut new_que = Vec::with_capacity(que.len() * 2);

            for node in que.iter() {
                level.push(node.borrow().val);
                if let Some(left) = node.borrow_mut().left.take() {
                    new_que.push(left);
                }
                if let Some(right) = node.borrow_mut().right.take() {
                    new_que.push(right);
                }
            }
            ret.push(level);
            que = new_que;
        }
        ret
    }
}