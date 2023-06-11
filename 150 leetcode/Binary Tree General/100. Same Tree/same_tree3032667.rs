// https://leetcode.com/problems/same-tree/solutions/3032667/rust-single-line/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        p == q
    }
}