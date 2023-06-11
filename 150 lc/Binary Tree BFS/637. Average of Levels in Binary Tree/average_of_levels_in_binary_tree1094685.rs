// https://leetcode.com/problems/average-of-levels-in-binary-tree/solutions/1094685/rust-bfs-solution/
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut vd = VecDeque::new();
        if let Some(r) = root {
            vd.push_back(r);
        }
        let mut answer = Vec::new();
        while !vd.is_empty() {
            let len = vd.len();
            let mut sum = 0;
            for _ in 0..vd.len() {
                if let Some(node) = vd.pop_front() {
                    sum += i64::from(node.borrow().val);
                    if let Some(n) = node.borrow_mut().left.take() {
                        vd.push_back(n);
                    }
                    if let Some(n) = node.borrow_mut().right.take() {
                        vd.push_back(n);
                    }
                }
            }
            answer.push(sum as f64 / len as f64);
        }
        answer
    }
}