// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/solutions/3529855/rust-faster-than-100-submissions-recursive-method/
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        return Self::sorted_array_to_bst_helper(&nums, 0, nums.len());
    }

    pub fn sorted_array_to_bst_helper(nums: &[i32], start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if(start >= end) {
            return None;
        }

        let index = (end - start) / 2 + start;
        
        let mut tree = TreeNode::new(nums[index]);
        tree.left = Self::sorted_array_to_bst_helper(nums, start, index);
        tree.right = Self::sorted_array_to_bst_helper(nums, index + 1, end);

        Some(Rc::new(RefCell::new(tree)))
    }
}