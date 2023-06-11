// https://leetcode.com/problems/average-of-levels-in-binary-tree/solutions/1381512/simple-bfs-in-rust-0ms/
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut res = Vec::new();

        let mut queue = VecDeque::new();
        queue.push_front(root.unwrap());

        while !queue.is_empty() {
            let count = queue.len();
            let mut acc = 0_f64;
            for _ in 0..count {
                let node = queue.pop_back().unwrap();
                acc += node.borrow().val as f64;

                if let Some(left) = &node.borrow().left {
                    queue.push_front(left.clone());
                };

                if let Some(right) = &node.borrow().right {
                    queue.push_front(right.clone());
                };
            }

            res.push(acc / count as f64);
        }

        res
    }
}