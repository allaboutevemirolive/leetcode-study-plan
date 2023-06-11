// https://leetcode.com/problems/symmetric-tree/solutions/2993486/rust-iterative-recursive/
use std::rc::Rc;
use std::cell::RefCell;
type NodeRef = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn is_symmetric(root: NodeRef) -> bool {
        if root.is_none() {
            return true;
        }
        fn is_symmetric_branch(left: &NodeRef, right: &NodeRef) -> bool {
            match (left, right) {
                (None, None) => true,
                (Some(p), Some(q)) => {
                    let TreeNode {
                        left: p_left,
                        right: p_right,
                        val: p_val,
                    } = &*p.borrow();
                    let TreeNode {
                        left: q_left,
                        right: q_right,
                        val: q_val,
                    } = &*q.borrow();
                    if p_val != q_val {
                        return false;
                    }

                    is_symmetric_branch(p_left, q_right) && is_symmetric_branch(q_left, p_right)
                }
                _ => false,
            }
        }

        let root_ref = root.unwrap();
        let root_node = &*root_ref.borrow();
        is_symmetric_branch(&root_node.left, &root_node.right)
    }
}