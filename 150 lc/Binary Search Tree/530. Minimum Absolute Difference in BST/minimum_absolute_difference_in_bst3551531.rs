// https://leetcode.com/problems/minimum-absolute-difference-in-bst/solutions/3551531/rust-with-bstiterator-from-173/
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::min;

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let vals = {
            let mut vals = vec![];
            let mut it = BSTIterator::new(root);
            while let Some(val) = it.next() {
                vals.push(val)
            }
            vals
        };

        let mut min = std::i32::MAX;
        for i in (0..vals.len()-1) {
            min = std::cmp::min(min, vals[i+1]-vals[i])
        }
        min
    }
}

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>
}

impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut s = Self{stack: Vec::new()};
        s.walk(root);
        s
    }
    
    fn next(&mut self) -> Option<i32> {
        let el = self.stack.pop();
        el.map(|el| {
            let mut el = el.borrow_mut();
            if let Some(right) = el.right.take() {
                self.walk(Some(right));
            }
            el.val
        })
    }
    
    fn walk(&mut self, root: Option<Rc<RefCell<TreeNode>>>) {
        let mut cur = root;
        while let Some(x) = cur {
            let l = x.borrow_mut().left.take();
            self.stack.push(x);
            cur = l;
        }
    }
    
    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}