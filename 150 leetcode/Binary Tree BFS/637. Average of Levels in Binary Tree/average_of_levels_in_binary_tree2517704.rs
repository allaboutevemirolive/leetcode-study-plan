// https://leetcode.com/problems/average-of-levels-in-binary-tree/solutions/2517704/rust-iterative-solution/
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut stack = vec![(0, root)];
        let mut sum: Vec<(usize, f64)> = vec![];
        while let Some((level, node)) = stack.pop() {
            if let Some(node) = node.as_ref().map(|n| n.borrow()) {
                match sum.get_mut(level) {
                    #[cfg(unstable)]
                    Some((n, sum)) => (*n, *sum) = (*n + 1, *sum + node.val as f64),
                    #[cfg(not(unstable))]
                    Some((n, sum)) => {
                        *n += 1;
                        *sum += node.val as f64;
                    }
                    _ => sum.push((1, node.val as f64)),
                }
                stack.push((level + 1, node.left.clone()));
                stack.push((level + 1, node.right.clone()));
            }
        }

        return sum.into_iter().map(|(n, sum)| sum / n as f64).collect();
    }
}