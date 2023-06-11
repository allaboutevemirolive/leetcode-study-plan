// https://leetcode.com/problems/invert-binary-tree/solutions/2281167/rust/
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref root) = root.clone(){
            let root = root.borrow();
            
            let mut new_node = TreeNode::new(root.val);
            new_node.left = Self::invert_tree(root.right.clone());
            new_node.right = Self::invert_tree(root.left.clone());
            
            Some(Rc::new(RefCell::new(new_node)))
        }else{
            None
        }
    }
}