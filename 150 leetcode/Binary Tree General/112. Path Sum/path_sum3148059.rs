// https://leetcode.com/problems/path-sum/solutions/3148059/rust-intuitive-recursive-approach/
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::traverse_sum(root, target_sum, 0)
    }

    fn traverse_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32, acc: i32) -> bool {
        match root {
            Some(node) => {
                let node = node.as_ref().borrow();

                if node.left.is_none() && node.right.is_none() {
                    return acc + node.val == target_sum;
                }

                Self::traverse_sum(node.left.clone(), target_sum, acc + node.val)
                    || Self::traverse_sum(node.right.clone(), target_sum, acc + node.val)
            }
            None => false,
        }
    }
}