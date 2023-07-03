// https://leetcode.com/problems/invert-binary-tree/solutions/3325794/rust-implementation/
use std::rc::Rc;
use std::cell::RefCell;
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn invert_tree(root: Node) -> Node {
        match root.clone() {
            Some(root_node) => {
                let mut node = root_node.borrow_mut();
                let temp = node.left.take();
                node.left = node.right.take();
                node.right = temp;
                Self::invert_tree(node.left.clone());
                Self::invert_tree(node.right.clone());
                root
            }
            None => None,
        }
    }
}