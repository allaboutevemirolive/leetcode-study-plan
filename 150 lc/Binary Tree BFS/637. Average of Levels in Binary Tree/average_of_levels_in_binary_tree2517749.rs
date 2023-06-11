// https://leetcode.com/problems/average-of-levels-in-binary-tree/solutions/2517749/rust-two-solutions-with-comments/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::iter::once;
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut q: VecDeque<_> = once(root).flatten().collect();
        let mut rez = vec![];
        while !q.is_empty() {
            let (mut sum, n) = (0.0, q.len());
            for _ in 0..n {
                let node_rc = q.pop_front().unwrap();
                let mut node_ref = node_rc.borrow_mut();
                sum += node_ref.val as f64;
                q.extend(once(node_ref.left.take()).chain(once(node_ref.right.take())).flatten());
            }
            rez.push(sum / (n as f64));
        }
        rez
    }
}