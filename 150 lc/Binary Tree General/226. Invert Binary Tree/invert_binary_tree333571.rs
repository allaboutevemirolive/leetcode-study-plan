// https://leetcode.com/problems/invert-binary-tree/solutions/333571/0ms-rust-solution/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn invert_helper(node: &mut Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            node.take().map(|n| {
                let left = invert_helper(&mut n.borrow_mut().left);
                let right = invert_helper(&mut n.borrow_mut().right);
                n.borrow_mut().left = right;
                n.borrow_mut().right = left;
                n
            })
        }
        
        invert_helper(&mut root)
    }
}