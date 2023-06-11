// https://leetcode.com/problems/invert-binary-tree/solutions/1055232/rust-iterative-solution/
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = Vec::new();
        let mut current_node = root.clone();
        loop {
            while let Some(node) = current_node {
                let mut borrowed_node = node.borrow_mut();

                // Can't use mem::swap because of thw borrow checker
                let old_left = borrowed_node.left.take();
                let old_right = std::mem::replace(&mut borrowed_node.right, old_left);
                borrowed_node.left = old_right;

                stack.push(borrowed_node.right.clone());
                current_node = borrowed_node.left.clone();
            }
            current_node = if let Some(node) = stack.pop() {
                node
            } else {
                break;
            }
        }
        root
    }
}