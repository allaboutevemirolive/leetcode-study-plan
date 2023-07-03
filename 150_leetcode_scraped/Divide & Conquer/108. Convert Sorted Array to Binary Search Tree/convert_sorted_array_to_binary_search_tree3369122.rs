// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/solutions/3369122/rust-implementation/
use std::rc::Rc;
use std::cell::RefCell;
type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Node {
        Self::create_bst(&nums, 0, nums.len())
    }

    fn create_bst(nums: &[i32], start: usize, end: usize) -> Node {
        if start < end {
            let root_index = (start + end) / 2;
            let mut root_node = TreeNode::new(nums[root_index]);

            root_node.left = Self::create_bst(nums, start, root_index);
            root_node.right = Self::create_bst(nums, root_index + 1, end);
            
            Some(Rc::new(RefCell::new(root_node)))
        } else {
            None
        }
    }
}