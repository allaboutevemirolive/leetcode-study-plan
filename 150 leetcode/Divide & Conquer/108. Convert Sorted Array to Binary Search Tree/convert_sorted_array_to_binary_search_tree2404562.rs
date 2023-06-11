// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/solutions/2404562/rust-yars-yet-another-recursive-solution/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_array_to_bst(mut nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let right = nums.split_off((nums.len() + 1) / 2);
        match nums.pop() {
            None => None,
            Some(val) => Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: Self::sorted_array_to_bst(nums),
                right: Self::sorted_array_to_bst(right),
            })))
        }
    }
}