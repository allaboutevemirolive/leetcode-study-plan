// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/solutions/3234554/rust-solution/
impl Solution {
    #[inline]
    fn _sorted_array_to_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let middle = nums.len() / 2;
        let mut leaf = TreeNode::new(nums[middle]);
        leaf.right = Self::_sorted_array_to_bst(&nums[middle + 1..]);
        leaf.left = Self::_sorted_array_to_bst(&nums[..middle]);
        Some(Rc::new(RefCell::new(leaf)))
    }

    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::_sorted_array_to_bst(&nums)
    }
}