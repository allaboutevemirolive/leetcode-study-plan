// https://leetcode.com/problems/minimum-absolute-difference-in-bst/solutions/1729549/rust-inorder-traversal-recursive-and-iterative/
use std::rc::Rc;
use std::cell::RefCell;
/// @author: Leon
/// https://leetcode.com/problems/minimum-absolute-difference-in-bst/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const RANGE: i32 = 1e5 as i32 + 7;
        let mut min_diff: i32 = RANGE;
        Self::inorder(root, &mut RANGE, &mut min_diff);
        min_diff
    }
    fn inorder(node: Option<Rc<RefCell<TreeNode>>>, prev: &mut i32, min_diff: &mut i32){
        if let Some(n) = node{
            Self::inorder(n.borrow().left.clone(), prev, min_diff);
            let value = n.borrow().val;
            *min_diff = std::cmp::min(*min_diff, (value - *prev).abs());
            *prev = value;
            Self::inorder(n.borrow().right.clone(), prev, min_diff);
        }
    }
}