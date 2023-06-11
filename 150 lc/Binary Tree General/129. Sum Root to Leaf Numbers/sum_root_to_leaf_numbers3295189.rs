// https://leetcode.com/problems/sum-root-to-leaf-numbers/solutions/3295189/javascript-rust-go-short-recursive-solution/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::sum_nodes(root, 0)
    }

    pub fn sum_nodes(node: Option<Rc<RefCell<TreeNode>>>, num: i32) -> i32 {
        if node.is_none() { return 0; }
        let node = node.unwrap();
        let node = node.borrow();
        let num = num * 10 + node.val;

        if node.left.is_none() && node.right.is_none() { return num; }

        Solution::sum_nodes(node.left.clone(), num.clone()) 
            + Solution::sum_nodes(node.right.clone(), num.clone())
    }
}