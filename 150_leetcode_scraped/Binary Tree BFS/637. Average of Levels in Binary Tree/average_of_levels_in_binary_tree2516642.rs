// https://leetcode.com/problems/average-of-levels-in-binary-tree/solutions/2516642/rust-one-level-at-a-time/
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut out: Vec<f64> = vec![];
        let mut active: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut future: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        active.push(root.unwrap());
        while !active.is_empty() {  // for each level in the tree
            let mut count = active.len() as f64;
            let mut total: f64 = 0.0;
            for n in active.drain(..) {
                let n = n.borrow();
                total += n.val as f64;
                if let Some(L) = &n.left { future.push(L.clone()); }
                if let Some(R) = &n.right { future.push(R.clone()); }
            }
            out.push(total / count);
            active.append(&mut future);
        }
        out
    }
}