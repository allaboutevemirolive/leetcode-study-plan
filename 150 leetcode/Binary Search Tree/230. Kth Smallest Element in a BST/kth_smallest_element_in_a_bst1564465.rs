// https://leetcode.com/problems/kth-smallest-element-in-a-bst/solutions/1564465/rust-in-order-traversal-iterator/
struct DepthFirstTraversal {
    root: Option<Rc<RefCell<TreeNode>>>,
    stack : Vec<Rc<RefCell<TreeNode>>>,
}

impl DepthFirstTraversal {
    fn from_root_node(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            root,
            stack: Vec::new()
        }
    }
}

impl Iterator for DepthFirstTraversal {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        while let Some(root) = self.root.take() {
            self.stack.push(root.clone());
            self.root = root.borrow().left.clone();
        }
        self.root = self.stack.pop();
        let val = self.root.as_ref().map(|node| node.borrow().val);
        self.root = self.root.as_ref().and_then(|node| node.borrow().right.clone());
        val
    }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        DepthFirstTraversal::from_root_node(root).nth((k - 1) as usize).unwrap_or(-1)
    }
}