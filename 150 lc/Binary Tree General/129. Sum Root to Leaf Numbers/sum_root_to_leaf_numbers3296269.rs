// https://leetcode.com/problems/sum-root-to-leaf-numbers/solutions/3296269/0-ms-idiomatic-rust-solution/
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;
impl Solution {
    #[inline]
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::helper(&root, 0)
    }

    #[inline]
    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, mut cur: i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            cur = (10 * cur) + node.val;

            if Solution::is_leaf(&node) {
                return cur;
            }

            return Solution::helper(&node.left, cur)
                + Solution::helper(&node.right, cur);
        }

        0
    }

    #[inline]
    fn is_leaf(node: &Ref<TreeNode>) -> bool {
        node.left.is_none() && node.right.is_none()
    }
}