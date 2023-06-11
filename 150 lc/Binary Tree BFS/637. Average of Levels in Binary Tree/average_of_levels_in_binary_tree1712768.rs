// https://leetcode.com/problems/average-of-levels-in-binary-tree/solutions/1712768/rust-solution/
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut res = Vec::new();
        let mut q = std::collections::VecDeque::new();
        q.push_back(root);

        while !q.is_empty() {
            let (mut s, mut n): (f64, f64) = (0.0, 0.0);
            for _ in 0..q.len() {
                if let Some(Some(node)) = q.pop_front() {
                    let node = node.borrow();
                    s += node.val as f64;
                    n += 1.0;
                    q.push_back(node.left.clone());
                    q.push_back(node.right.clone());
                }
            }
            if n > 0.0 {
                res.push(s / n);
            }
        }
        res
    }
}