// https://leetcode.com/problems/average-of-levels-in-binary-tree/solutions/2516684/rust-dfs-solution/
use std::rc::Rc;
use std::cell::RefCell;
type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn average_of_levels(root: OptNode) -> Vec<f64> {
        let mut avr = Vec::new();
        Self::calc_avr(&root, 0, &mut avr, &mut Vec::new());
        avr
    }

    fn calc_avr(node: &OptNode, depth: usize, avr: &mut Vec<f64>, count: &mut Vec<f64>) {
        if let Some(n) = node.as_ref() {
            if depth == avr.len() {
                avr.push(0.0);
                count.push(0.0);
            }
            let b = n.borrow();
            avr[depth] = (avr[depth] * count[depth] + (b.val as f64)) / (count[depth] + 1.0);
            count[depth] += 1.0;
            Self::calc_avr(&b.left, depth + 1, avr, count);
            Self::calc_avr(&b.right, depth + 1, avr, count);
        }
    }
}