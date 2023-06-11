// https://leetcode.com/problems/sum-root-to-leaf-numbers/solutions/3296698/rust-c-dfs-post-order/
use std::{rc::Rc, cell::RefCell};

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn sum_numbers(root: OptNode) -> i32 {
        let mut sum = 0;
        Solution::post_order_leaf_sum(root, 0, &mut sum);

        sum
    }

    fn post_order_leaf_sum(node: OptNode, mut digits: i32, leaf_sum: &mut i32) {
        if let Some(n) = node {
            let n = n.borrow();
            digits *= 10;
            digits += n.val;

            Solution::post_order_leaf_sum(n.left.clone(), digits, leaf_sum);
            Solution::post_order_leaf_sum(n.right.clone(), digits, leaf_sum);

            // If we are at a leaf node, add the digits to the sum.
            if n.left.is_none() && n.right.is_none() {
                (*leaf_sum) += digits;
            }
        }
    }
}