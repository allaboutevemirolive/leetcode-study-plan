// https://leetcode.com/problems/average-of-levels-in-binary-tree/solutions/1094706/average-of-levels-in-binary-tree-rust/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque; // bi-directional queue
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        
        let mut pc_level: u32 = 0;
        let mut next: VecDeque<(u32, Option<Rc<RefCell<TreeNode>>>)> = VecDeque::new();
        next.push_back((pc_level, root));
        let mut total: f64 = 0.0;
        let mut level_count: f64 = 0.0;
        let mut result: Vec<f64> = Vec::new();
        
        while next.len() > 0 {
            if let Some((l, Some(n))) = next.pop_front() {
                let n = n.borrow();
                if l == pc_level {
                    total += n.val as f64;
                    level_count += 1.0;
                } else {
                    pc_level += 1;
                    result.push(total / level_count);
                    total = n.val as f64;
                    level_count = 1.0;
                }
                next.push_back((pc_level + 1, n.left.clone()));
                next.push_back((pc_level + 1, n.right.clone()));
            } // don't care about 'else' conditions None and (l, None)
        }
        result.push(total / level_count);
        
        result
    }
}