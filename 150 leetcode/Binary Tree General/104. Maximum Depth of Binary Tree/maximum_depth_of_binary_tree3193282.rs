// https://leetcode.com/problems/maximum-depth-of-binary-tree/solutions/3193282/rust-more-idiomatic-solution-o-n-0ms/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<&TreeNode>, cur_depth: usize) -> usize {
            if root.is_none() {
                return cur_depth;
            }

            let root = root.unwrap();
            let l = root.left.as_ref().map(|x| x.borrow());
            let r = root.right.as_ref().map(|x| x.borrow());

            let l_val = dfs(l.as_deref(), cur_depth + 1);
            let r_val = dfs(r.as_deref(), cur_depth + 1);

            l_val.max(r_val)
        }

        let r = root.as_ref().map(|x| x.borrow());
        dfs(r.as_deref(), 0) as i32
    }
}
