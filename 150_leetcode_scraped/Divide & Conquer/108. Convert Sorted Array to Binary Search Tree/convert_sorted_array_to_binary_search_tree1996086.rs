// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/solutions/1996086/rust-recursion/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&nums, 0 as i32, nums.len() as i32 - 1)
    }
    
    pub fn helper(nums: &Vec<i32>, l: i32, r: i32) -> Option<Rc<RefCell<TreeNode>>>{
        if l > r {
            return None;
        }
        
        let mid = (l+r)/2;
        let mut root = TreeNode::new(nums[mid as usize]);
        root.left = Self::helper(nums, l, mid-1);
        root.right = Self::helper(nums, mid+1, r);
        
        Some(Rc::new(RefCell::new(root)))
    }
}
