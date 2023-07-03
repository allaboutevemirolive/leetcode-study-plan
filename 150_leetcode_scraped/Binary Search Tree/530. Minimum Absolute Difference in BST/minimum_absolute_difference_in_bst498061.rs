// https://leetcode.com/problems/minimum-absolute-difference-in-bst/solutions/498061/rust-solution/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::cmp;
        let mut min = i32::max_value();
        let mut before = -1;
        fn scan_tree(tree:Option<Rc<RefCell<TreeNode>>>,curr_min:&mut i32,curr_before:&mut i32) {
            if tree.is_none(){
                return;
            }
            let tree = tree.unwrap();
            let tree_node = tree.borrow();
            let left_child = tree_node.left.clone();
            let val = tree_node.val;
            let right_child = tree_node.right.clone();
            if left_child.is_some(){
                scan_tree(left_child,curr_min,curr_before);
            }
            if *curr_before != -1 {
                *curr_min = (*curr_min).min(val - *curr_before);
            }
            *curr_before = val;
            if right_child.is_some(){
                scan_tree(right_child,curr_min,curr_before);
            }
        }
        scan_tree(root,&mut min,&mut before);
        min
    }
