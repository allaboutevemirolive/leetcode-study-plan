// https://leetcode.com/problems/binary-search-tree-iterator/solutions/2533276/rust-simple-solution/
use std::cell::RefCell;
use std::rc::Rc;

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut bst_iterator = Self { stack: Vec::new() };
        bst_iterator.expand_left(root);
        bst_iterator
    }
    
    fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();
        let val = node.borrow().val;
        self.expand_left(node.borrow().right.clone());
        val
    }
    
    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
    
    fn expand_left(&mut self, root: Option<Rc<RefCell<TreeNode>>>) {
        let mut root = root;
        while let Some(node) = root {
            self.stack.push(node.clone());
            root = node.borrow().left.clone();
        }
    }
}