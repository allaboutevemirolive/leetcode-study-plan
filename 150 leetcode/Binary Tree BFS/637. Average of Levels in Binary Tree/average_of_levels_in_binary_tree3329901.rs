// https://leetcode.com/problems/average-of-levels-in-binary-tree/solutions/3329901/rust-implementation-100/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut average: Vec<f64> = vec![];
        let mut queue = Vec::new();
        if let Some(root) = root {
            queue.push(root);
        }
        while !queue.is_empty() {
            let mut items = Vec::new();
            let mut total: f64 = 0.0;
            let count = queue.len() as f64;
            while let Some(node) = queue.pop() {
                let current_node = node.borrow();
                total += current_node.val as f64;
                items.extend(current_node.left.clone());
                items.extend(current_node.right.clone());
            }
            average.push(total / count as f64);
            queue.extend(items);
        }
        average
    }
}