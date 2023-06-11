// https://leetcode.com/problems/invert-binary-tree/solutions/478919/rust/
impl Solution {
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::h(&mut root);
        root
    }

    fn h(r: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(r) = r {
            let mut r = r.borrow_mut();
            Self::h(&mut r.left);
            Self::h(&mut r.right);

            let lt = std::mem::replace(&mut r.left, None);
            let rt = std::mem::replace(&mut r.right, lt);
            std::mem::replace(&mut r.left, rt);
        }
    }
}