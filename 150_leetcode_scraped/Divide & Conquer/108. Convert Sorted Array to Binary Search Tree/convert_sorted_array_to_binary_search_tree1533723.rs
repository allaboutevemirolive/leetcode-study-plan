// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/solutions/1533723/rust-using-match/
use std::rc::Rc;
use std::cell::RefCell;

type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Node {
        let mut n = nums.len();
        
        match n { 
             0 => None,
            _ => { 
                let m = n/2;
                let mut node = TreeNode::new(nums[m]);
                node.left = Self::sorted_array_to_bst(nums[..m].to_vec());
                node.right = Self::sorted_array_to_bst(nums[m + 1..].to_vec());
                
                Some(Rc::new(RefCell::new(node)))
            }
        }
    }
}
	// update: you can still do it without using match