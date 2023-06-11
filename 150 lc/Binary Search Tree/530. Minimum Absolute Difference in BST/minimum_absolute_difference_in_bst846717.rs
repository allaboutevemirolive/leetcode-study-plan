// https://leetcode.com/problems/minimum-absolute-difference-in-bst/solutions/846717/rust-cheapest-best/
use std::i32::{MAX, MIN};

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::go(&root, MIN, MAX).1
    }

    fn go(root: &Option<Rc<RefCell<TreeNode>>>, last: i32, res: i32) -> (i32, i32) {
        match root {
            Some(node) => {
                let b = node.borrow();
                let (mut last, mut res) = Self::go(&b.left, last, res);
                res = res.min(b.val.saturating_sub(last));
                last = b.val;
                Self::go(&b.right, last, res)
            }
            _ => (last, res),
        }
    }
}