// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/solutions/2404623/rust-recursive-simple/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::bst(&nums)
    }
	
    fn bst(v: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        match v.len() {
            0 => None,
            1 => Some(Rc::new(RefCell::new(TreeNode::new(v[0])))),
            len => {
                let mid_idx = len / 2;
                let mut new_node = TreeNode::new(v[mid]);
                new_node.left = Solution::bst(&v[..mid_idx]);
                new_node.right = Solution::bst(&v[(mid_idx+1)..]);
                Some(Rc::new(RefCell::new(new_node)))
            }
        }
    }
}