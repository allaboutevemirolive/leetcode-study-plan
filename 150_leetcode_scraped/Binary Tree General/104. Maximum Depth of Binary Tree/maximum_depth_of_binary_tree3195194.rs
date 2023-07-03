// https://leetcode.com/problems/maximum-depth-of-binary-tree/solutions/3195194/rust-0ms-2-5-mb/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_height = 1;
        let mut stack = vec![];

        match root {
            None => return 0,
            Some(node) => stack.push((node.clone(), max_height)),
        }

        while stack.len() > 0 {
            let (node, height) = stack.pop().unwrap();

            if height > max_height { max_height = height; }

            if let Some(left) = node.borrow().left.clone() {
                stack.push((left, height + 1));
            }

            if let Some(right) = node.borrow().right.clone() {
                stack.push((right, height + 1));
            };
        }

        max_height
    }
}
