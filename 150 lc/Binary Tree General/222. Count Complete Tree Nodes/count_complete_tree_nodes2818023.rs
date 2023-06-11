// https://leetcode.com/problems/count-complete-tree-nodes/solutions/2818023/rust-clean-code-o-log-n-log-n/
use std::rc::Rc;
use std::cell::RefCell;
type Leaf = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::count_leaves(&root)
    }

    fn count_leaves(leaf: &Leaf) -> i32 {
        if let Some(l) = leaf  {
            // Let's calculate height of the max length branch 
            // (left branch is always longer or equal to right)
            let left_count = Solution::add_left(&leaf);
            let right_count = Solution::add_right(&leaf);
            
            // This means that subtree that we counting now is also complete tree
            // Number of leaves in this tree is 2^height -1 
            // Height we calculated by traversing tree to the furthermost left leaf
            if left_count == right_count {
                return (1 << left_count) - 1
            }
            
            // Here we are counting the right subtree and left subtree recursively
            return 1 + Solution::count_leaves(&l.as_ref().borrow().left) + Solution::count_leaves(&l.as_ref().borrow().right)
        } 
        0
    }

    fn add_left(leaf: &Leaf) -> i32 {
        if let Some(l) = leaf  {
            return 1 + Solution::add_left(&l.as_ref().borrow().left)
        }
        0
    }

    fn add_right(leaf: &Leaf) -> i32 {
        if let Some(l) = leaf  {
            return 1 + Solution::add_right(&l.as_ref().borrow().right)
        }
        0
    }
}