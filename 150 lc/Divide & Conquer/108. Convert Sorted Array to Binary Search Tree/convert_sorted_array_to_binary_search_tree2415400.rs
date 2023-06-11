// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/solutions/2415400/rust-slice-patterns/
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::sorted_slice_to_bst(&nums)
    }

    fn sorted_slice_to_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if let (left, [val, right @ ..]) = nums.split_at(nums.len() / 2) {
            let mut node = TreeNode::new(*val);
            node.left = Self::sorted_slice_to_bst(left);
            node.right = Self::sorted_slice_to_bst(right);

            Some(Rc::new(RefCell::new(node)))
        } else {
            None
        }
    }