// https://leetcode.com/problems/validate-binary-search-tree/solutions/2435883/rust-recursive-solution-with-comments-0ms/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::is_valid_bst_helper(root, None, None)
    }
    
	// low is a (exclusive) lower bound for the node value
	// high is a (exclusive) upper bound for the node value
    pub fn is_valid_bst_helper(root: Option<Rc<RefCell<TreeNode>>>, low: Option<i32>, high: Option<i32>) -> bool {
        match root {
            None => true,
            Some(node) => {
                let val = node.borrow().val;
                if !low.is_none() && val <= low.unwrap() || !high.is_none() && val >= high.unwrap() {
					// node value is outside of the bounds, so the bst is invalid
                    false
                } else {
					// the bst is valid if both subtrees are valid
					// parent node value is upper bound for left child and lower bound for right child
                    Solution::is_valid_bst_helper(node.borrow().left.clone(), low, Some(val))
                        && Solution::is_valid_bst_helper(node.borrow().right.clone(), Some(val), high)
                }
            }
        }
    }
}