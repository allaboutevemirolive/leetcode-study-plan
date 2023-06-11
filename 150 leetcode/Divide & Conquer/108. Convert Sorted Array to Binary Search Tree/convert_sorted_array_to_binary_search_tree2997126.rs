// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/solutions/2997126/rust-iterative-recursive-0ms-100-simple/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let (smaller_half, larger_half) = nums.split_at(nums.len() / 2);
        let root_rc = Rc::new(RefCell::new(TreeNode::new(larger_half[0])));

        root_rc.borrow_mut().left = Self::sorted_array_to_bst(smaller_half.to_vec());
        root_rc.borrow_mut().right = Self::sorted_array_to_bst(larger_half[1..].to_vec());
        Some(root_rc)
    }
}