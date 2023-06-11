// https://leetcode.com/problems/invert-binary-tree/solutions/1758159/rust/
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(node: &mut Option<Rc<RefCell<TreeNode>>>) {
            node.as_ref().map(|n| {
                let mut borrowed = n.borrow_mut();
                let left = borrowed.left.take();
                let right = borrowed.right.take();
                right.map(|r| borrowed.left.replace(r));
                left.map(|l| borrowed.right.replace(l));
                helper(&mut borrowed.left);
                helper(&mut borrowed.right);
            });
        }

        helper(&mut root);
        root
    }