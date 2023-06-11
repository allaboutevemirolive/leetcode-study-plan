// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/solutions/1208091/rust-recursive-solution-with-no-clone/
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let mut bm = node.borrow_mut();
            Self::flatten(&mut bm.left);
            if let Some(left) = bm.left.take() {
                let val = left.borrow().val;
                bm.right = Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: left.borrow_mut().right.take(),
                    right: bm.right.take(),
                })));
            }
            Self::flatten(&mut bm.right);
        }
    }
}