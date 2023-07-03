// https://leetcode.com/problems/symmetric-tree/solutions/3341135/rust-implementation/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn traverse(
            left: &Option<Rc<RefCell<TreeNode>>>,
            right: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (left, right) {
                (None, None) => true,
                (Some(left_ref), Some(right_ref)) => {
                    let left_node = left_ref.borrow();
                    let right_node = right_ref.borrow();
                    if left_node.val == right_node.val {
                        traverse(&left_node.left, &right_node.right)
                            && traverse(&left_node.right, &right_node.left)
                    } else {
                        false
                    }
                }
                (None, Some(_)) | (Some(_), None) => false,
            }
        }
        match root {
            Some(root_ref) => {
                let root_node = root_ref.borrow();
                traverse(&root_node.left, &root_node.right)
            }
            None => false,
        }
    }
}
