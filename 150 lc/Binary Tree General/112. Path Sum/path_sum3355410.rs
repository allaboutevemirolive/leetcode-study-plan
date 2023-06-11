// https://leetcode.com/problems/path-sum/solutions/3355410/rust-implementation/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        fn traverse(
            root: &Option<Rc<RefCell<TreeNode>>>,
            current_sum: i32,
            target_sum: i32,
        ) -> bool {
            match root {
                Some(root_ref) => {
                    let root_node = root_ref.borrow();

                    let left_node = &root_node.left;
                    let right_node = &root_node.right;

                    if left_node.is_none() && right_node.is_none() {
                        if target_sum == current_sum + root_node.val {
                            return true;
                        }
                    }
                    traverse(left_node, current_sum + root_node.val, target_sum)
                        || traverse(right_node, current_sum + root_node.val, target_sum)
                }
                None => false,
            }
        }
        traverse(&root, 0, target_sum)
    }
}
