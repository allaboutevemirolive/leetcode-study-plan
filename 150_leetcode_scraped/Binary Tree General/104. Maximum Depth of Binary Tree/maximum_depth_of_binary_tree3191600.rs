// https://leetcode.com/problems/maximum-depth-of-binary-tree/solutions/3191600/compact-iterative-dfs-with-pattern-matching-in-rust/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (mut ans, mut stack) = (0, vec![(root, 0)]);
        while let Some((node, depth)) = stack.pop() {
            match node {
                None => ans = ans.max(depth),
                Some(node) => {
                    let (TreeNode { left, right, .. }, depth) = (&*node.borrow(), depth + 1);
                    stack.push((left.clone(), depth));
                    stack.push((right.clone(), depth));
                }
            }
        }
        ans
    }
}