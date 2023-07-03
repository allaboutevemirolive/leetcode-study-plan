// https://leetcode.com/problems/sum-root-to-leaf-numbers/solutions/3295337/simlpe-rust-recursive-solution/
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(root) => Self::collect_numbers(root, 0),
            None => 0,
        }
    }

    fn collect_numbers(root: Rc<RefCell<TreeNode>>, number: i32) -> i32 {
        let root = root.borrow();
        let number = number * 10 + root.val;

        match (&root.left, &root.right) {
            (None, None) => number,
            (None, Some(right)) => Self::collect_numbers(right.clone(), number),
            (Some(left), None) => Self::collect_numbers(left.clone(), number),
            (Some(left), Some(right)) => {
                Self::collect_numbers(right.clone(), number)
                    + Self::collect_numbers(left.clone(), number)
            }
        }
    }
}