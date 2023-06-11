// https://leetcode.com/problems/invert-binary-tree/solutions/1748032/rust-0ms-2mb-update-2022/
use std::rc::Rc;
use std::cell::RefCell;
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn invert_tree(root: Node) -> Node {
        if let Some(node) = root.clone() { 
            let mut node = node.borrow_mut();
            let (l, r) = (
                Self::invert_tree(node.left.clone()),
                Self::invert_tree(node.right.clone())
            );
            node.left = r;
            node.right = l;
        }
        root
    }
}