// https://leetcode.com/problems/minimum-absolute-difference-in-bst/solutions/1918023/rust-solution/
use std::cell::RefCell;
use std::rc::Rc;
type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn get_minimum_difference(root: OptNode) -> i32 {
        fn in_order_traverse(node: &OptNode, v: &mut Vec<i32>) {
            if let Some(node) = node {
                let node = node.borrow();
                in_order_traverse(&node.left, v);
                v.push(node.val);
                in_order_traverse(&node.right, v);
            }
        }

        let mut min_diff = i32::MAX;
        let mut v = Vec::new();
        in_order_traverse(&root, &mut v);

        v.iter()
            .zip(v[1..].iter())
            .for_each(|(&x, &y)| min_diff = min_diff.min(y - x));

        min_diff
    }
}