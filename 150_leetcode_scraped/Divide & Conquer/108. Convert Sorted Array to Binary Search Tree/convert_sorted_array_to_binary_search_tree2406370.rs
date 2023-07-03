// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/solutions/2406370/rust-efficient-with-comments/
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn recurse(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            None
        } else {
            let (left, rest) = nums.split_at(nums.len() / 2);
            let (curr, right) = rest.split_first().unwrap();
            Some(Rc::new(RefCell::new(TreeNode {
                val: *curr,
                left: Self::recurse(left),
                right: Self::recurse(right),
            })))
        }
    }

    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::recurse(&nums)
    }
}