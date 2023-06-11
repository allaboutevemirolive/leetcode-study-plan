// https://leetcode.com/problems/validate-binary-search-tree/solutions/2552776/rust-0ms-faster-than-100/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        
        fn isValid(root: Option<Rc<RefCell<TreeNode>>>, low: Option<i32>, high: Option<i32>)-> bool {
            match root {
                None => 
                    return true,
                Some(node) => {
                    let n = node.borrow();
                    let v = Some(n.val);
                    if (!low.is_none() && v <= low) || (!high.is_none() && v >= high) {
                        return false;
                    }
                    
                    isValid(n.left.clone(), low, v) && isValid(n.right.clone(), v, high)
                }
            }            
        }
        
        isValid(root, None, None)
    }
}